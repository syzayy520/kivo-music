use super::super::{PlaybackManager, PlaybackManagerState};

#[test]
fn manager_diagnostic_default_manager_returns_disabled_snapshot() {
    let manager = PlaybackManager::new();
    let snapshot = manager.tap_diagnostic_report_snapshot();

    assert!(!snapshot.enabled);
    assert!(snapshot.current.is_none());
    assert!(snapshot.last_detached.is_none());
    assert!(snapshot.current_stream.is_none());
}

#[test]
fn manager_diagnostic_default_manager_state_returns_disabled_snapshot_without_error() {
    let manager_state = PlaybackManagerState::default();
    let snapshot = manager_state.tap_diagnostic_report_snapshot();

    assert!(!snapshot.enabled);
    assert!(snapshot.current.is_none());
    assert!(snapshot.last_detached.is_none());
    assert!(snapshot.current_stream.is_none());
}

#[test]
fn manager_diagnostic_enabled_policy_without_active_tap_returns_empty_snapshot(
) -> Result<(), String> {
    let manager = diagnostic_manager(4)?;
    let snapshot = manager.tap_diagnostic_report_snapshot();

    assert!(snapshot.enabled);
    assert!(snapshot.current.is_none());
    assert!(snapshot.last_detached.is_none());
    assert!(snapshot.current_stream.is_none());
    Ok(())
}

fn diagnostic_manager(capacity_frames: u32) -> Result<PlaybackManager, String> {
    let config =
        crate::playback::native_pipeline_route_tap_diagnostic_policy::NativeTapDiagnosticConfig::new(
            capacity_frames,
        )
        .map_err(|error| format!("{error:?}"))?;
    let mut manager = PlaybackManager::new();
    manager.primary_engine =
        crate::playback::backends::native::KivoNativeEngine::new_with_tap_diagnostic_policy(config);
    Ok(manager)
}
