use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSink};

use super::submit_error_observation::{
    observe_submit_frame_error, NativePipelineDrainObservedSubmitError,
};

pub(in crate::playback) fn observe_submit_frame_result(
    result: PlaybackResult<OutputRuntimeStatus>,
) -> Result<OutputRuntimeStatus, NativePipelineDrainObservedSubmitError> {
    result.map_err(observe_submit_frame_error)
}

impl NativePipeline {
    pub(in crate::playback) fn submit_drained_output_frame(
        &mut self,
        frame: AudioOutputFrame,
    ) -> Result<OutputRuntimeStatus, NativePipelineDrainObservedSubmitError> {
        observe_submit_frame_result(self.output.submit_frame(frame))
    }
}
