use crate::playback::audio_route_pipeline_tap::{
    AudioRoutePipelineTap, AudioRoutePipelineTapConfig, AudioRoutePipelineTapReport,
};
use crate::playback::decoder::AudioStreamInfo;
use crate::playback::native_pipeline::NativePipeline;

use super::error::NativeTapDiagnosticPolicyError;
use super::state::NativePipelineRouteTapDiagnosticPolicy;

impl NativePipelineRouteTapDiagnosticPolicy {
    pub fn open_new_track(
        &mut self,
        pipeline: &mut NativePipeline,
        stream: AudioStreamInfo,
    ) -> Result<AudioRoutePipelineTapReport, NativeTapDiagnosticPolicyError> {
        self.close_detach_current(pipeline);
        let config = AudioRoutePipelineTapConfig::new(
            stream.clone(),
            self.config.diagnostic_route_capacity_frames,
        )
        .map_err(NativeTapDiagnosticPolicyError::TapCreation)?;
        let tap = AudioRoutePipelineTap::new(config)
            .map_err(NativeTapDiagnosticPolicyError::TapCreation)?;
        let report = tap.report();
        pipeline.attach_audio_route_pipeline_tap(tap);
        self.current_stream = Some(stream);
        Ok(report)
    }
}
