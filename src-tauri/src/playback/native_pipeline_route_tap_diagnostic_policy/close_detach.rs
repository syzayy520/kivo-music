use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::native_pipeline::NativePipeline;

use super::state::NativePipelineRouteTapDiagnosticPolicy;

impl NativePipelineRouteTapDiagnosticPolicy {
    pub fn close_detach_on_stop(
        &mut self,
        pipeline: &mut NativePipeline,
    ) -> Option<AudioRoutePipelineTapReport> {
        self.close_detach_current(pipeline)
    }

    pub fn close_detach_on_close_decoder(
        &mut self,
        pipeline: &mut NativePipeline,
    ) -> Option<AudioRoutePipelineTapReport> {
        self.close_detach_current(pipeline)
    }

    pub fn close_detach_on_shutdown(
        &mut self,
        pipeline: &mut NativePipeline,
    ) -> Option<AudioRoutePipelineTapReport> {
        self.close_detach_current(pipeline)
    }

    pub(super) fn close_detach_current(
        &mut self,
        pipeline: &mut NativePipeline,
    ) -> Option<AudioRoutePipelineTapReport> {
        let report = pipeline
            .detach_audio_route_pipeline_tap()
            .map(|mut tap| tap.close());
        if let Some(report) = report.as_ref() {
            self.last_detached_report = Some(report.clone());
        }
        self.current_stream = None;
        report
    }
}
