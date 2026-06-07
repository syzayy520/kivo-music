use super::super::decoder_runtime_state::DecoderRuntimeState;
use super::super::decoder_session::DecoderSession;
use super::super::native_pipeline::NativePipeline;
use super::super::native_pipeline_buffer::NativePipelineBuffer;
use super::super::native_pipeline_clock::NativePipelineClock;
use super::super::output::AudioOutputFrame;

pub(super) struct NativePipelineSeekSnapshot {
    decoder_session: Option<DecoderSession>,
    decoder_state: DecoderRuntimeState,
    last_decoded_frame: Option<AudioOutputFrame>,
    buffer: NativePipelineBuffer,
    clock: NativePipelineClock,
}

impl NativePipelineSeekSnapshot {
    /// Capture observable wrapper state without mutating the pipeline.
    ///
    /// All fields are cloned — no drain, take, clear, or replace.
    pub(super) fn capture(pipeline: &NativePipeline) -> Self {
        Self {
            decoder_session: pipeline.state.decoder_session.clone(),
            decoder_state: pipeline.state.decoder_state.clone(),
            last_decoded_frame: pipeline.state.last_decoded_frame.clone(),
            buffer: pipeline.buffer.clone(),
            clock: pipeline.clock.clone(),
        }
    }

    pub(super) fn decoder_session(&self) -> &Option<DecoderSession> {
        &self.decoder_session
    }

    pub(super) fn decoder_state(&self) -> &DecoderRuntimeState {
        &self.decoder_state
    }

    pub(super) fn last_decoded_frame(&self) -> &Option<AudioOutputFrame> {
        &self.last_decoded_frame
    }

    pub(super) fn buffer(&self) -> &NativePipelineBuffer {
        &self.buffer
    }

    pub(super) fn clock(&self) -> &NativePipelineClock {
        &self.clock
    }
}
