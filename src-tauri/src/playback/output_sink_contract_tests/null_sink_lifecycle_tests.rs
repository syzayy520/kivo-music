use super::super::output::{OutputRuntimeStatus, OutputSink};
use super::fixtures::{null_sink, output_frame, output_settings};

fn assert_same_status(actual: &OutputRuntimeStatus, expected: &OutputRuntimeStatus) {
    assert_eq!(actual.is_open, expected.is_open);
    assert_eq!(actual.is_active, expected.is_active);
    assert_eq!(
        actual.active_device_id.as_deref(),
        expected.active_device_id.as_deref()
    );
    assert_eq!(actual.pending_frames, expected.pending_frames);
    assert_eq!(actual.latency.requested_ms, expected.latency.requested_ms);
    assert_eq!(actual.latency.measured_ms, expected.latency.measured_ms);
    assert_eq!(actual.controls.volume_level, expected.controls.volume_level);
    assert_eq!(actual.controls.muted, expected.controls.muted);
    assert_eq!(actual.gap_count, expected.gap_count);
    assert_eq!(actual.last_error.as_deref(), expected.last_error.as_deref());
}

#[test]
fn null_sink_lifecycle_open_sets_current_active_status() {
    let mut sink = null_sink();

    let status = sink
        .open(&output_settings(Some("null-lifecycle-device")))
        .expect("open should succeed");

    assert!(status.is_open);
    assert!(status.is_active);
    assert_eq!(
        status.active_device_id.as_deref(),
        Some("null-lifecycle-device")
    );
    assert_eq!(status.pending_frames, 0);
    assert!(status.last_error.is_none());
    assert_same_status(&sink.status(), &status);
}

#[test]
fn null_sink_lifecycle_pause_and_resume_return_current_status() {
    let mut sink = null_sink();
    sink.open(&output_settings(None))
        .expect("open should succeed");
    sink.submit_frame(output_frame(10))
        .expect("submit should succeed");
    let before_pause = sink.status();

    let paused = sink.pause().expect("pause should succeed");
    let resumed = sink.resume().expect("resume should succeed");

    assert_same_status(&paused, &before_pause);
    assert_same_status(&resumed, &before_pause);
}

#[test]
fn null_sink_lifecycle_flush_clears_pending_frames_only() {
    let mut sink = null_sink();
    sink.open(&output_settings(Some("null-flush-device")))
        .expect("open should succeed");
    sink.submit_frame(output_frame(10))
        .expect("first submit should succeed");
    sink.submit_frame(output_frame(20))
        .expect("second submit should succeed");

    let status = sink.flush().expect("flush should succeed");

    assert!(status.is_open);
    assert!(status.is_active);
    assert_eq!(
        status.active_device_id.as_deref(),
        Some("null-flush-device")
    );
    assert_eq!(status.pending_frames, 0);
    assert!(status.last_error.is_none());
}

#[test]
fn null_sink_lifecycle_stop_deactivates_and_clears_pending_frames() {
    let mut sink = null_sink();
    sink.open(&output_settings(None))
        .expect("open should succeed");
    sink.submit_frame(output_frame(10))
        .expect("submit should succeed");

    let status = sink.stop().expect("stop should succeed");

    assert!(status.is_open);
    assert!(!status.is_active);
    assert_eq!(status.pending_frames, 0);
    assert!(status.last_error.is_none());
}

#[test]
fn null_sink_lifecycle_close_resets_status_to_default() {
    let mut sink = null_sink();
    sink.open(&output_settings(Some("null-close-device")))
        .expect("open should succeed");
    sink.submit_frame(output_frame(10))
        .expect("submit should succeed");

    sink.close().expect("close should succeed");
    let status = sink.status();

    assert_same_status(&status, &OutputRuntimeStatus::default());
}
