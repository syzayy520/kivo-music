use crate::playback::decoder::AudioStreamInfo;
use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::decoder_runtime_state::DecoderRuntimeState;
use crate::playback::decoder_session::DecoderSession;
use crate::playback::decoders::factory::create_decoder_for_path;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::AudioOutputFrame;

impl NativePipeline {
    pub fn set_decoder_request(&mut self, request: AudioDecoderOpenRequest) {
        self.state.decoder_request = Some(request);
    }

    pub fn open_decoder(
        &mut self,
        request: AudioDecoderOpenRequest,
        opened_at_ms: u64,
    ) -> PlaybackResult<DecoderSession> {
        self.state.decoder_state.begin_opening();
        self.state.decoder_request = Some(request.clone());
        self.state.decoder_session = None;
        self.state.last_decoded_frame = None;
        self.decoder = None;

        let mut decoder = create_decoder_for_path(&request.source_path).inspect_err(|error| {
            self.state.decoder_state.mark_failed(error.to_string());
        })?;

        let stream_info = decoder.open(&request.source_path).inspect_err(|error| {
            self.state.decoder_state.mark_failed(error.to_string());
        })?;
        let duration_ms = decoder.duration_ms();

        let session = DecoderSession::from_open_request_with_duration(
            &request,
            stream_info,
            opened_at_ms,
            duration_ms,
        );
        self.state.decoder_session = Some(session.clone());
        self.state.decoder_state.mark_open();
        self.decoder = Some(decoder);

        Ok(session)
    }

    pub fn configure_decoder_open(
        &mut self,
        request: AudioDecoderOpenRequest,
        stream_info: AudioStreamInfo,
        opened_at_ms: u64,
    ) {
        self.state.decoder_state.begin_opening();
        self.state.decoder_request = Some(request.clone());
        self.state.decoder_session = Some(DecoderSession::from_open_request(
            &request,
            stream_info,
            opened_at_ms,
        ));
        self.state.decoder_state.mark_open();
    }

    pub fn update_decoder_position(&mut self, position_ms: u64) {
        if let Some(session) = self.state.decoder_session.as_mut() {
            session.update_position(position_ms);
        }
    }

    pub fn count_decoded_frame(&mut self) {
        if let Some(session) = self.state.decoder_session.as_mut() {
            session.count_frame();
        }
    }

    pub fn set_decoder_state(&mut self, state: DecoderRuntimeState) {
        self.state.decoder_state = state;
    }

    pub fn schedule_decode_step(&mut self) -> PlaybackResult<()> {
        let decoder = self.decoder.as_mut().ok_or_else(|| {
            PlaybackError::Backend("native pipeline decoder is not open".to_string())
        })?;

        let frame = decoder.next_frame().inspect_err(|error| {
            self.state.decoder_state.mark_failed(error.to_string());
        })?;

        match frame {
            Some(frame) => {
                self.update_decoder_position(frame.position_ms);
                self.count_decoded_frame();
                let output_frame = AudioOutputFrame {
                    stream: frame.stream,
                    position_ms: frame.position_ms,
                    samples: frame.samples,
                };
                self.tap_audio_route_output_frame_non_fatal(&output_frame);
                self.state.last_decoded_frame = Some(output_frame.clone());
                self.enqueue_decoded_frame(output_frame);
                Ok(())
            }
            None => {
                self.state.decoder_state.begin_draining();
                self.state.last_decoded_frame = None;
                Ok(())
            }
        }
    }

    pub fn seek_decoder(&mut self, position_ms: u64) -> PlaybackResult<()> {
        crate::playback::native_pipeline_seek_transaction::seek_decoder_transaction(
            self,
            position_ms,
        )
    }

    pub fn close_decoder(&mut self) -> PlaybackResult<()> {
        if let Some(decoder) = self.decoder.as_mut() {
            decoder.close()?;
        }

        self.decoder = None;
        self.state.decoder_session = None;
        self.state.last_decoded_frame = None;
        self.state.decoder_state.mark_closed();
        self.clear_buffer();
        Ok(())
    }
}
