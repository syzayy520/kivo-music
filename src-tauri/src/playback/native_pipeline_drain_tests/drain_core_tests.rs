use crate::playback::errors::PlaybackError;
use crate::playback::native_pipeline::NativePipeline;

use super::helpers::test_frame;

#[test]
fn drain_empty_buffer_returns_backend_error() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.drain_next_frame_to_output();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "pipeline buffer is empty, no frame to drain");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn drain_submits_frame_to_null_sink_and_updates_pending() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(0));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.last_error.is_none());
    assert!(pipeline.buffer.is_empty());
}

#[test]
fn drain_success_updates_clock_position_to_drained_frame_position() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(500));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");

    assert_eq!(pipeline.clock.position_ms(), 500);
    assert!(pipeline.clock.is_started());
}

#[test]
fn drain_success_starts_clock_if_not_started() {
    let mut pipeline = NativePipeline::new();
    pipeline.enqueue_decoded_frame(test_frame(1200));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");

    assert_eq!(pipeline.clock.position_ms(), 1200);
    assert!(pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
}

#[test]
fn drain_empty_buffer_does_not_change_clock() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    let result = pipeline.drain_next_frame_to_output();
    assert!(result.is_err());

    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
}

#[test]
fn drain_sequential_drains_update_clock_position() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(100));
    pipeline
        .drain_next_frame_to_output()
        .expect("first drain should succeed");
    assert_eq!(pipeline.clock.position_ms(), 100);

    pipeline.enqueue_decoded_frame(test_frame(200));
    pipeline
        .drain_next_frame_to_output()
        .expect("second drain should succeed");
    assert_eq!(pipeline.clock.position_ms(), 200);

    pipeline.enqueue_decoded_frame(test_frame(500));
    pipeline
        .drain_next_frame_to_output()
        .expect("third drain should succeed");
    assert_eq!(pipeline.clock.position_ms(), 500);
}
