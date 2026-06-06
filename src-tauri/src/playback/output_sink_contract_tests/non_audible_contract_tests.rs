use super::super::output::{OutputRuntimeStatus, OutputSink};
use super::fixtures::{null_sink, output_frame, output_settings};

fn assert_no_runtime_output_observables(status: &OutputRuntimeStatus) {
    assert_eq!(status.latency.requested_ms, None);
    assert_eq!(status.latency.measured_ms, None);
    assert_eq!(status.gap_count, 0);
    assert!(status.last_error.is_none());
}

#[test]
fn non_audible_contract_submit_success_is_consumption_only() {
    let mut sink = null_sink();

    let status = sink
        .submit_frame(output_frame(700))
        .expect("current null sink submit should consume frame");

    assert_eq!(status.pending_frames, 1);
    assert!(!status.is_open);
    assert!(!status.is_active);
    assert_no_runtime_output_observables(&status);
}

#[test]
fn non_audible_contract_open_tracks_only_configured_boundary_identity() {
    let mut sink = null_sink();

    let status = sink
        .open(&output_settings(Some("null-boundary-only")))
        .expect("open should record current boundary identity");

    assert_eq!(
        status.active_device_id.as_deref(),
        Some("null-boundary-only")
    );
    assert_eq!(status.pending_frames, 0);
    assert_no_runtime_output_observables(&status);
}

#[test]
fn non_audible_contract_close_returns_default_observable_status() {
    let mut sink = null_sink();
    sink.open(&output_settings(Some("null-boundary-only")))
        .expect("open should succeed");
    sink.submit_frame(output_frame(800))
        .expect("submit should succeed");

    sink.close().expect("close should reset status");
    let status = sink.status();

    assert_eq!(status.is_open, OutputRuntimeStatus::default().is_open);
    assert_eq!(status.is_active, OutputRuntimeStatus::default().is_active);
    assert_eq!(
        status.pending_frames,
        OutputRuntimeStatus::default().pending_frames
    );
    assert_eq!(
        status.active_device_id.as_deref(),
        OutputRuntimeStatus::default().active_device_id.as_deref()
    );
    assert_no_runtime_output_observables(&status);
}
