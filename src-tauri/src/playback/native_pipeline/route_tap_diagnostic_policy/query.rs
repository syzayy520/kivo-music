use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::decoder::AudioStreamInfo;
use crate::playback::native_pipeline::NativePipeline;

use super::state::NativePipelineRouteTapDiagnosticPolicy;
use super::stream_compat::is_same_diagnostic_route_stream;

impl NativePipelineRouteTapDiagnosticPolicy {
    pub fn current_tap_report(
        &self,
        pipeline: &NativePipeline,
    ) -> Option<AudioRoutePipelineTapReport> {
        pipeline.audio_route_pipeline_tap_report()
    }

    pub fn last_detached_report(&self) -> Option<AudioRoutePipelineTapReport> {
        self.last_detached_report.clone()
    }

    pub fn current_stream(&self) -> Option<&AudioStreamInfo> {
        self.current_stream.as_ref()
    }

    pub fn current_stream_is_compatible(&self, stream: &AudioStreamInfo) -> bool {
        self.current_stream
            .as_ref()
            .is_some_and(|current| is_same_diagnostic_route_stream(current, stream))
    }
}
