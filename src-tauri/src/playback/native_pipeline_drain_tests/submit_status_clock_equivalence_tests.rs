use crate::playback::native_pipeline::NativePipeline;

use super::helpers::test_frame;

#[test]
fn submit_status_clock_success_keeps_status_and_clock_semantics() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(320));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should submit through output sink");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.is_open);
    assert!(state.output_status.is_active);
    assert!(state.output_status.last_error.is_none());
    assert_eq!(pipeline.clock.position_ms(), 320);
    assert!(pipeline.clock.is_started());
    assert!(pipeline.buffer.is_empty());
}

#[test]
fn submit_status_clock_success_starts_clock_when_not_started() {
    let mut pipeline = NativePipeline::new();

    pipeline.enqueue_decoded_frame(test_frame(640));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should start clock at frame position");

    assert_eq!(pipeline.clock.position_ms(), 640);
    assert!(pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
}

#[test]
fn submit_status_clock_started_clock_uses_set_position_semantics() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(100));
    pipeline
        .drain_next_frame_to_output()
        .expect("first drain should set position");
    pipeline.enqueue_decoded_frame(test_frame(900));
    pipeline
        .drain_next_frame_to_output()
        .expect("second drain should set position again");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 2);
    assert_eq!(pipeline.clock.position_ms(), 900);
    assert!(pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
    assert!(pipeline.buffer.is_empty());
}
