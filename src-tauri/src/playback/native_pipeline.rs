use std::fmt;

use super::decoder::AudioDecoder;
use super::native_output::KivoNativeOutputSink;
pub use super::native_pipeline_state::NativePipelineState;
use super::output::OutputSink;

#[derive(Default)]
pub struct NativePipeline {
    pub(super) state: NativePipelineState,
    pub(super) decoder: Option<Box<dyn AudioDecoder>>,
    pub(super) output: KivoNativeOutputSink,
}

impl fmt::Debug for NativePipeline {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativePipeline")
            .field("state", &self.state)
            .field("decoder_open", &self.decoder.is_some())
            .field("output_status", &self.output.status())
            .finish()
    }
}

impl NativePipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> NativePipelineState {
        self.state.clone()
    }
}
