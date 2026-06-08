use std::fs;

use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::native_pipeline::NativePipeline;

use super::helpers::{test_frame, write_long_test_wav, write_test_wav};

#[test]
fn schedule_decode_step_pushes_frame_to_buffer() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-buffer-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");

    assert_eq!(pipeline.buffered_frame_count(), 1);
    let state = pipeline.snapshot();
    assert!(state.last_decoded_frame.is_some());

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn seek_decoder_clears_buffer() {
    let path = write_long_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-buffer-seek-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    pipeline.seek_decoder(2).expect("seek decoder");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 2);

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn shutdown_clears_buffer_and_resets_clock() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-buffer-shutdown-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    pipeline.shutdown().expect("shutdown pipeline");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(!pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());

    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn stop_output_clears_buffer_and_resets_clock() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(0));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");
    assert_eq!(pipeline.clock.position_ms(), 0);

    pipeline.enqueue_decoded_frame(test_frame(500));
    pipeline.stop_output().expect("stop output");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(!pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
}

#[test]
fn flush_output_clears_buffer_but_keeps_clock_position() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(300));
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");
    assert_eq!(pipeline.clock.position_ms(), 300);

    pipeline.enqueue_decoded_frame(test_frame(600));
    pipeline.flush_output().expect("flush output");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 300);
    assert!(pipeline.clock.is_started());
}
