use std::fs;

use super::helpers::*;
use crate::playback::decoder_runtime_state::DecoderRuntimePhase;
use crate::playback::native_pipeline_loop::NativePipelineLoopStep;

#[test]
fn run_until_blocked_respects_max_steps() {
    let path = write_long_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline.run_until_blocked(1).expect("run_until_blocked(1)");
    let pending_after_1 = pipeline.snapshot().output_status.pending_frames;
    assert_eq!(pending_after_1, 1);

    pipeline.run_until_blocked(0).expect("run_until_blocked(0)");
    let pending_after_0 = pipeline.snapshot().output_status.pending_frames;
    assert_eq!(pending_after_0, pending_after_1);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn run_until_blocked_stops_at_end_of_stream() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline
        .run_until_blocked(1000)
        .expect("run_until_blocked should stop at EOS");

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Draining);
    assert_eq!(pipeline.buffered_frame_count(), 0);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn seek_clears_buffer_before_loop_resumes() {
    let path = write_long_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline
        .decode_once_to_buffer()
        .expect("decode should succeed");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    pipeline.seek_decoder(5).expect("seek");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 5);

    let step = pipeline.pump_once().expect("pump after seek");
    assert_eq!(step, NativePipelineLoopStep::Drained);

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn shutdown_after_loop_closes_pipeline_and_resets_clock() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline.pump_once().expect("pump_once");
    assert!(pipeline.clock.is_started());
    assert!(pipeline.buffered_frame_count() == 0);

    pipeline.shutdown().expect("shutdown");

    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(!pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());
    assert_eq!(pipeline.buffered_frame_count(), 0);

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(!state.output_status.is_open);

    fs::remove_file(path).expect("remove wav");
}
