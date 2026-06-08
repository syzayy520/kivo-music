use std::fs;

use super::wav_test_file::write_long_test_wav;
use crate::playback::errors::PlaybackError;
use crate::playback::native_pipeline::NativePipeline;

use super::*;

fn open_pipeline_with_decoded_frame() -> (NativePipeline, String) {
    let path = write_long_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "seek-tx-test".to_string(),
        source_path: path.clone(),
    };
    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline.schedule_decode_step().expect("decode first frame");
    (pipeline, path)
}

// ── Success-path tests ──────────────────────────────────────────────

#[test]
fn seek_transaction_success_commits_decoder_position() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    let before_session = pipeline.snapshot().decoder_session.unwrap();
    assert_eq!(before_session.last_position_ms, 0);

    pipeline.seek_decoder(2).expect("seek should succeed");

    let after_session = pipeline.snapshot().decoder_session.unwrap();
    assert_eq!(after_session.last_position_ms, 2);

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_success_clears_last_decoded_frame() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    assert!(
        pipeline.snapshot().last_decoded_frame.is_some(),
        "should have decoded frame before seek"
    );

    pipeline.seek_decoder(2).expect("seek should succeed");

    assert!(
        pipeline.snapshot().last_decoded_frame.is_none(),
        "last_decoded_frame should be None after successful seek"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_success_clears_buffer() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    assert_eq!(
        pipeline.buffer.len(),
        1,
        "buffer should have 1 frame after decode"
    );

    pipeline.seek_decoder(2).expect("seek should succeed");

    assert_eq!(
        pipeline.buffer.len(),
        0,
        "buffer should be empty after successful seek"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_success_updates_clock() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    assert_eq!(pipeline.clock.position_ms(), 0, "clock should start at 0");

    pipeline.seek_decoder(2).expect("seek should succeed");

    assert_eq!(
        pipeline.clock.position_ms(),
        2,
        "clock should reflect seek position"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

// ── Decoder-not-open test ───────────────────────────────────────────

#[test]
fn seek_transaction_decoder_not_open_no_mutation() {
    let mut pipeline = NativePipeline::new();
    let state_before = pipeline.snapshot();

    let result = pipeline.seek_decoder(1_000);

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline decoder is not open");
        }
        other => panic!("expected Backend error, got {other:?}"),
    }

    let state_after = pipeline.snapshot();
    assert_eq!(
        state_after.decoder_state.phase, state_before.decoder_state.phase,
        "decoder_state phase should not change"
    );
    assert!(
        state_after.decoder_session.is_none(),
        "decoder_session should remain None"
    );
    assert!(
        state_after.last_decoded_frame.is_none(),
        "last_decoded_frame should remain None"
    );
    assert_eq!(pipeline.buffer.len(), 0, "buffer should remain empty");
    assert_eq!(pipeline.clock.position_ms(), 0, "clock should not change");
}

// ── Failure-path tests ──────────────────────────────────────────────

#[test]
fn seek_transaction_decoder_seek_failure_restores_decoder_state() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    let state_before = pipeline.snapshot();
    let session_before = state_before.decoder_session.as_ref().unwrap();
    let session_track_id = session_before.track_id.clone();
    let session_position = session_before.last_position_ms;
    let session_frame_count = session_before.decoded_frame_count;
    let decoder_phase_before = state_before.decoder_state.phase.clone();
    let decoder_error_before = state_before.decoder_state.last_error.clone();

    // u64::MAX triggers "seek target exceeds wav range" in WAV decoder
    let result = pipeline.seek_decoder(u64::MAX);

    assert!(result.is_err(), "seek with u64::MAX should fail");

    let state_after = pipeline.snapshot();
    let session_after = state_after.decoder_session.as_ref().unwrap();
    assert_eq!(
        session_after.track_id, session_track_id,
        "session track_id should be restored"
    );
    assert_eq!(
        session_after.last_position_ms, session_position,
        "session position should be restored"
    );
    assert_eq!(
        session_after.decoded_frame_count, session_frame_count,
        "session frame count should be restored"
    );
    assert_eq!(
        state_after.decoder_state.phase, decoder_phase_before,
        "decoder_state phase should be restored"
    );
    assert_eq!(
        state_after.decoder_state.last_error, decoder_error_before,
        "decoder_state last_error should be restored"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_failure_does_not_clear_buffer() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    let buffer_len_before = pipeline.buffer.len();
    assert!(
        buffer_len_before > 0,
        "buffer should have frames before seek"
    );

    let result = pipeline.seek_decoder(u64::MAX);
    assert!(result.is_err(), "seek with u64::MAX should fail");

    assert_eq!(
        pipeline.buffer.len(),
        buffer_len_before,
        "buffer length should be unchanged after failed seek"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_failure_does_not_clear_last_frame() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    assert!(
        pipeline.snapshot().last_decoded_frame.is_some(),
        "should have decoded frame before seek"
    );

    let result = pipeline.seek_decoder(u64::MAX);
    assert!(result.is_err(), "seek with u64::MAX should fail");

    assert!(
        pipeline.snapshot().last_decoded_frame.is_some(),
        "last_decoded_frame should be preserved after failed seek"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}

#[test]
fn seek_transaction_failure_does_not_move_clock() {
    let (mut pipeline, path) = open_pipeline_with_decoded_frame();
    let clock_before = pipeline.clock.position_ms();

    let result = pipeline.seek_decoder(u64::MAX);
    assert!(result.is_err(), "seek with u64::MAX should fail");

    assert_eq!(
        pipeline.clock.position_ms(),
        clock_before,
        "clock position should be unchanged after failed seek"
    );

    pipeline.shutdown().expect("shutdown");
    let _ = fs::remove_file(path);
}
