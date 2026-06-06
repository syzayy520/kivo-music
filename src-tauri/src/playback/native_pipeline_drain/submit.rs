use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSink};

impl NativePipeline {
    pub(in crate::playback) fn submit_drained_output_frame(
        &mut self,
        frame: AudioOutputFrame,
    ) -> PlaybackResult<OutputRuntimeStatus> {
        self.output.submit_frame(frame)
    }
}
