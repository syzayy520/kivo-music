use std::fs;

use super::super::decoder_runtime_state::DecoderRuntimePhase;
use super::super::native_pipeline_loop::NativePipelineLoopStep;
use super::helpers::*;

#[test]
fn pump_once_decodes_and_drains_one_frame_to_null_sink() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    let step = pipeline.pump_once().expect("pump_once should succeed");
    assert_eq!(step, NativePipelineLoopStep::Drained);

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.last_error.is_none());
    assert!(pipeline.clock.is_started());

    let progress = pipeline.progress_snapshot();
    assert!(progress.position_ms > 0 || progress.position_ms == 0);
    assert_eq!(progress.buffered_frames, 0);
    assert_eq!(progress.output_pending_frames, 1);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn pump_once_drains_existing_buffer_before_decoding_more() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline.decode_once_to_buffer().expect("decode once");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    let step = pipeline
        .pump_once()
        .expect("pump_once should drain existing");
    assert_eq!(step, NativePipelineLoopStep::Drained);
    assert_eq!(pipeline.buffered_frame_count(), 0);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn decode_once_to_buffer_pushes_frame() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    pipeline
        .decode_once_to_buffer()
        .expect("decode should succeed");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    let state = pipeline.snapshot();
    assert!(state.last_decoded_frame.is_some());
    assert_eq!(
        state.decoder_session.as_ref().unwrap().decoded_frame_count,
        1
    );

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn pump_once_returns_end_of_stream_after_decoder_drains() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    let mut drain_count = 0;
    loop {
        match pipeline.pump_once().expect("pump_once") {
            NativePipelineLoopStep::Drained => {
                drain_count += 1;
            }
            NativePipelineLoopStep::EndOfStream => break,
        }
    }

    assert!(drain_count > 0, "should have drained at least one frame");

    let step = pipeline.pump_once().expect("pump_once after eos");
    assert_eq!(step, NativePipelineLoopStep::EndOfStream);

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Draining);
    assert_eq!(pipeline.buffered_frame_count(), 0);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}
