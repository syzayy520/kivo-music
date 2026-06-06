use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::decoder::AudioStreamInfo;

use super::config::NativeTapDiagnosticConfig;

#[derive(Debug)]
pub struct NativePipelineRouteTapDiagnosticPolicy {
    pub(super) config: NativeTapDiagnosticConfig,
    pub(super) current_stream: Option<AudioStreamInfo>,
    pub(super) last_detached_report: Option<AudioRoutePipelineTapReport>,
}

impl NativePipelineRouteTapDiagnosticPolicy {
    pub fn new(config: NativeTapDiagnosticConfig) -> Self {
        Self {
            config,
            current_stream: None,
            last_detached_report: None,
        }
    }
}
