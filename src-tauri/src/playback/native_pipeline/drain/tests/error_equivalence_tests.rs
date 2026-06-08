use std::path::Path;

use crate::playback::errors::PlaybackError;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::{OutputRuntimeStatus, OutputSink};

fn playback_source(path: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("playback")
            .join(path),
    )
    .expect("read playback source")
}

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
fn native_pipeline_drain_empty_buffer_error_keeps_status_unchanged() {
    let mut pipeline = NativePipeline::new();
    let before = pipeline.snapshot().output_status;

    let result = pipeline.drain_next_frame_to_output();

    assert!(matches!(result, Err(PlaybackError::Backend(_))));
    let after = pipeline.snapshot().output_status;
    assert_same_status(&after, &before);
}

#[test]
fn native_pipeline_drain_submit_error_refresh_helper_uses_output_status_snapshot() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    let stale_status = OutputRuntimeStatus {
        pending_frames: 99,
        last_error: Some("stale drain status".to_string()),
        ..OutputRuntimeStatus::default()
    };
    pipeline.set_output_status(stale_status);

    let expected = pipeline.output.status();
    pipeline.refresh_drain_submit_error_status();
    let actual = pipeline.snapshot().output_status;

    assert_same_status(&actual, &expected);
}

#[test]
fn native_pipeline_drain_submit_error_handler_keeps_refresh_then_err_source() {
    let root = playback_source("native_pipeline_drain.rs");
    let error = playback_source("native_pipeline_drain/error.rs");

    assert!(root.contains("Err(error) => error::handle_drain_submit_error(self, error)"));
    assert!(error.contains("pipeline.refresh_drain_submit_error_status();"));
    assert!(error.contains("Err(error)"));
}
