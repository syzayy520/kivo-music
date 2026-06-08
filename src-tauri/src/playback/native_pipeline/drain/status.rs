use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::{OutputRuntimeStatus, OutputSink};

impl NativePipeline {
    pub(in crate::playback) fn apply_drain_submit_success_status(
        &mut self,
        status: OutputRuntimeStatus,
    ) {
        self.state.output_status = status;
    }

    pub(in crate::playback) fn refresh_drain_submit_error_status(&mut self) {
        self.state.output_status = self.output.status();
    }
}
