use super::super::output::OutputSink;
use super::fixtures::{null_sink, output_frame, output_settings};

#[test]
fn submit_frame_contract_accepts_frame_without_opening_sink() {
    let mut sink = null_sink();

    let status = sink
        .submit_frame(output_frame(125))
        .expect("submit_frame should use current consumption contract");

    assert!(!status.is_open);
    assert!(!status.is_active);
    assert_eq!(status.pending_frames, 1);
    assert!(status.active_device_id.is_none());
    assert!(status.last_error.is_none());
    assert_eq!(sink.status().pending_frames, 1);
}

#[test]
fn submit_frame_contract_counts_each_submitted_frame() {
    let mut sink = null_sink();
    sink.open(&output_settings(Some("null-contract-device")))
        .expect("open current null sink");

    let first = sink
        .submit_frame(output_frame(0))
        .expect("first submit_frame should succeed");
    let second = sink
        .submit_frame(output_frame(250))
        .expect("second submit_frame should succeed");

    assert_eq!(first.pending_frames, 1);
    assert_eq!(second.pending_frames, 2);
    assert_eq!(sink.status().pending_frames, 2);
    assert_eq!(
        second.active_device_id.as_deref(),
        Some("null-contract-device")
    );
    assert!(second.is_open);
    assert!(second.is_active);
    assert!(second.last_error.is_none());
}

#[test]
fn submit_frame_contract_preserves_existing_control_and_latency_fields() {
    let mut sink = null_sink();
    sink.open(&output_settings(None))
        .expect("open current null sink");

    let status = sink
        .submit_frame(output_frame(500))
        .expect("submit_frame should not change unrelated status fields");

    assert_eq!(status.controls.volume_level, 1.0);
    assert!(!status.controls.muted);
    assert_eq!(status.latency.requested_ms, None);
    assert_eq!(status.latency.measured_ms, None);
    assert_eq!(status.gap_count, 0);
    assert!(status.last_error.is_none());
}
