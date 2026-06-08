pub(in crate::playback) mod buffer;
pub(in crate::playback) mod clock;
pub(in crate::playback) mod decoder;
pub(in crate::playback) mod drain;
pub(in crate::playback) mod loop_step;
pub(in crate::playback) mod output;
pub(in crate::playback) mod progress;
pub(in crate::playback) mod route_tap;
pub mod route_tap_diagnostic_policy;
pub(in crate::playback) mod runtime;
pub(in crate::playback) mod seek_transaction;
pub(in crate::playback) mod state;
pub(in crate::playback) mod worker;

#[cfg(test)]
mod tests;

use std::fmt;

use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTap;
use crate::playback::decoder::AudioDecoder;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::native_pipeline_buffer::NativePipelineBuffer;
use crate::playback::native_pipeline_clock::NativePipelineClock;
pub(in crate::playback) use crate::playback::native_pipeline_state::NativePipelineState;
use crate::playback::output::OutputSink;

pub struct NativePipeline {
    pub(super) state: NativePipelineState,
    pub(super) decoder: Option<Box<dyn AudioDecoder>>,
    pub(super) output: KivoNativeOutputSink,
    pub(super) buffer: NativePipelineBuffer,
    pub(super) clock: NativePipelineClock,
    pub(super) route_tap: Option<AudioRoutePipelineTap>,
}

impl Default for NativePipeline {
    fn default() -> Self {
        Self {
            state: NativePipelineState::default(),
            decoder: None,
            output: KivoNativeOutputSink::default(),
            buffer: NativePipelineBuffer::new(),
            clock: NativePipelineClock::new(),
            route_tap: None,
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
