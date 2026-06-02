use super::decoder::AudioStreamInfo;
use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::DecoderRuntimeState;
use super::decoder_session::DecoderSession;
use super::decoders::factory::create_decoder_for_path;
use super::errors::{PlaybackError, PlaybackResult};
use super::native_pipeline::NativePipeline;
use super::output::AudioOutputFrame;

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

        let session = DecoderSession::from_open_request(&request, stream_info, opened_at_ms);
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
                self.state.last_decoded_frame = Some(AudioOutputFrame {
                    stream: frame.stream,
                    position_ms: frame.position_ms,
                    samples: frame.samples,
                });
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
        let decoder = self.decoder.as_mut().ok_or_else(|| {
            PlaybackError::Backend("native pipeline decoder is not open".to_string())
        })?;

        decoder.seek(position_ms).inspect_err(|error| {
            self.state.decoder_state.mark_failed(error.to_string());
        })?;

        self.update_decoder_position(position_ms);
        self.state.last_decoded_frame = None;
        Ok(())
    }

    pub fn close_decoder(&mut self) -> PlaybackResult<()> {
        if let Some(decoder) = self.decoder.as_mut() {
            decoder.close()?;
        }

        self.decoder = None;
        self.state.decoder_session = None;
        self.state.last_decoded_frame = None;
        self.state.decoder_state.mark_closed();
        Ok(())
    }
}
