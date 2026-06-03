use std::fmt;

use super::decoder::AudioDecoder;
use super::native_output::KivoNativeOutputSink;
use super::native_pipeline_buffer::NativePipelineBuffer;
use super::native_pipeline_clock::NativePipelineClock;
pub(in crate::playback) use super::native_pipeline_state::NativePipelineState;
use super::output::OutputSink;

pub struct NativePipeline {
    pub(super) state: NativePipelineState,
    pub(super) decoder: Option<Box<dyn AudioDecoder>>,
    pub(super) output: KivoNativeOutputSink,
    pub(super) buffer: NativePipelineBuffer,
    pub(super) clock: NativePipelineClock,
}

impl Default for NativePipeline {
    fn default() -> Self {
        Self {
            state: NativePipelineState::default(),
            decoder: None,
            output: KivoNativeOutputSink::default(),
            buffer: NativePipelineBuffer::new(),
            clock: NativePipelineClock::new(),
        }
    }
}

impl fmt::Debug for NativePipeline {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativePipeline")
            .field("state", &self.state)
            .field("decoder_open", &self.decoder.is_some())
            .field("output_status", &self.output.status())
            .field("buffer_len", &self.buffer.len())
            .field("clock_position_ms", &self.clock.position_ms())
            .finish()
    }
}

impl NativePipeline {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub(in crate::playback) fn snapshot(&self) -> NativePipelineState {
        self.state.clone()
    }
}
