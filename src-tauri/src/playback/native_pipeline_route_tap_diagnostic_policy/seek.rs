use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::native_pipeline::NativePipeline;

use super::error::NativeTapDiagnosticPolicyError;
use super::state::NativePipelineRouteTapDiagnosticPolicy;

impl NativePipelineRouteTapDiagnosticPolicy {
    pub fn reset_on_seek(
        &mut self,
        pipeline: &mut NativePipeline,
    ) -> Result<Option<AudioRoutePipelineTapReport>, NativeTapDiagnosticPolicyError> {
        pipeline
            .reset_audio_route_pipeline_tap()
            .map_err(NativeTapDiagnosticPolicyError::TapReset)
    }
}
