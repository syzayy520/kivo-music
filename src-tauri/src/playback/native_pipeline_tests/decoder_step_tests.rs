use std::fs;

use super::wav_test_file::{write_long_test_wav, write_test_wav};
use super::*;

#[test]
fn schedule_decode_step_records_first_wav_frame() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-wav-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 25).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");

    let state = pipeline.state();
    let session = state
        .decoder_session
        .expect("decoder session should stay open");
    let frame = state
        .last_decoded_frame
        .expect("decoded frame should be captured");

    assert_eq!(session.decoded_frame_count, 1);
    assert_eq!(session.last_position_ms, 0);
    assert_eq!(frame.stream.sample_rate_hz, 48_000);
    assert_eq!(frame.stream.channels, 2);
    assert_eq!(frame.samples.len(), 8);

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn schedule_output_submit_step_routes_decoded_frame_to_output_boundary() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-wav-output-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 25).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");

    let result = pipeline.schedule_output_submit_step();

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                "kivo native output submit frame is not implemented yet"
            );
        }
        Err(other) => panic!("expected unsupported operation, got {other}"),
        Ok(_) => panic!("expected unsupported output submit, got success"),
    }

    let state = pipeline.state();
    assert_eq!(
        state.output_status.last_error.as_deref(),
        Some("kivo native output submit frame is not implemented yet")
    );

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn seek_decoder_moves_session_position_and_clears_decoded_frame() {
    let path = write_long_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-wav-seek-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 25).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");

    pipeline.seek_decoder(2).expect("seek decoder");

    let state = pipeline.state();
    let session = state
        .decoder_session
        .expect("decoder session should stay open");

    assert_eq!(session.last_position_ms, 2);
    assert!(state.last_decoded_frame.is_none());

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn shutdown_closes_open_decoder_and_output_state() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "track-wav-shutdown-1".to_string(),
        source_path: path.clone(),
    };

    pipeline.open_decoder(request, 25).expect("open decoder");
    pipeline
        .schedule_decode_step()
        .expect("schedule decode step");
    pipeline.shutdown().expect("shutdown pipeline");

    let state = pipeline.state();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(state.decoder_session.is_none());
    assert!(state.last_decoded_frame.is_none());
    assert_eq!(state.output_status.pending_frames, 0);

    fs::remove_file(path).expect("remove wav test file");
}
