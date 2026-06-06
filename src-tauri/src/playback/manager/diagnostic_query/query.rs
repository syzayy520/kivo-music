use super::snapshot::TapDiagnosticReportSnapshot;
use crate::playback::manager::{PlaybackManager, PlaybackManagerState};

impl PlaybackManager {
    pub(in crate::playback) fn tap_diagnostic_report_snapshot(
        &self,
    ) -> TapDiagnosticReportSnapshot {
        TapDiagnosticReportSnapshot::from_native_engine(&self.primary_engine)
    }
}

impl PlaybackManagerState {
    pub(in crate::playback) fn tap_diagnostic_report_snapshot(
        &self,
    ) -> TapDiagnosticReportSnapshot {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.tap_diagnostic_report_snapshot()
    }
}
