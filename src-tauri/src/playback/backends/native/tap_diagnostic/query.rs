use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::decoder::AudioStreamInfo;

use super::super::KivoNativeEngine;

impl KivoNativeEngine {
    pub(in crate::playback) fn tap_diagnostic_is_enabled(&self) -> bool {
        self.tap_diagnostic.policy().is_some()
    }

    pub(in crate::playback) fn tap_diagnostic_current_report(
        &self,
    ) -> Option<AudioRoutePipelineTapReport> {
        self.tap_diagnostic
            .policy()
            .and_then(|policy| policy.current_tap_report(&self.pipeline))
    }

    pub(in crate::playback) fn tap_diagnostic_last_detached_report(
        &self,
    ) -> Option<AudioRoutePipelineTapReport> {
        self.tap_diagnostic
            .policy()
            .and_then(|policy| policy.last_detached_report())
    }

    pub(in crate::playback) fn tap_diagnostic_current_stream(&self) -> Option<&AudioStreamInfo> {
        self.tap_diagnostic
            .policy()
            .and_then(|policy| policy.current_stream())
    }

    pub(in crate::playback) fn tap_diagnostic_stream_is_compatible(
        &self,
        stream: &AudioStreamInfo,
    ) -> bool {
        self.tap_diagnostic
            .policy()
            .is_some_and(|policy| policy.current_stream_is_compatible(stream))
    }
}
