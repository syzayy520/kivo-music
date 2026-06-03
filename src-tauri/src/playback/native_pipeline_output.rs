use super::native_pipeline::NativePipeline;
#[cfg(test)]
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};

impl NativePipeline {
    #[cfg(test)]
    pub(in crate::playback) fn set_output_settings(&mut self, settings: OutputSettings) {
        self.state.output_settings = settings;
    }

    #[cfg(test)]
    pub(in crate::playback) fn set_output_status(&mut self, status: OutputRuntimeStatus) {
        self.state.output_status = status;
    }

    #[cfg(test)]
    pub(in crate::playback) fn note_frame_submitted(&mut self, _frame: &AudioOutputFrame) {
        self.state.output_status.pending_frames =
            self.state.output_status.pending_frames.saturating_add(1);
        self.state.output_status.is_active = true;
    }
}
