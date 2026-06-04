use std::fs;

use super::super::errors::PlaybackError;
use super::super::native_pipeline::NativePipeline;
use super::helpers::*;

#[test]
fn drain_once_to_output_submits_and_updates_clock() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline
        .decode_once_to_buffer()
        .expect("decode should succeed");
    pipeline
        .drain_once_to_output()
        .expect("drain should succeed");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(pipeline.clock.is_started());

    let progress = pipeline.progress_snapshot();
    assert_eq!(progress.buffered_frames, 0);
    assert_eq!(progress.output_pending_frames, 1);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn drain_once_to_output_returns_error_on_empty_buffer() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");

    let result = pipeline.drain_once_to_output();
    assert!(result.is_err());

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "pipeline buffer is empty, no frame to drain");
        }
        other => panic!("expected backend error, got {other:?}"),
    }

    assert_eq!(pipeline.clock.position_ms(), 0);

    pipeline.shutdown().expect("shutdown");
}

#[test]
fn pump_once_returns_error_on_decoder_not_open() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.pump_once();
    assert!(result.is_err());

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline decoder is not open");
        }
        other => panic!("expected backend error, got {other:?}"),
    }
}

#[test]
fn loop_does_not_update_clock_on_failed_drain() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");

    let result = pipeline.drain_once_to_output();
    assert!(result.is_err());

    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(pipeline.clock.is_started());

    pipeline.shutdown().expect("shutdown");
}
