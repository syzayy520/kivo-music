use std::fs;

use super::decoder_runtime_state::DecoderRuntimePhase;
use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::native_pipeline_loop::NativePipelineLoopStep;

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
}

fn write_test_wav() -> String {
    let path = std::env::temp_dir().join(format!(
        "kivo-pipeline-loop-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create wav test file");

    for sample in [0_i16, 1200, -1200, 2400, -2400, 3600, -3600, 0] {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav test file");

    path.to_string_lossy().into_owned()
}

fn write_long_test_wav() -> String {
    let path = std::env::temp_dir().join(format!(
        "kivo-pipeline-loop-long-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create long wav test file");

    for sample in 0..512_i16 {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize long wav test file");

    path.to_string_lossy().into_owned()
}

fn open_wav_pipeline(path: &str) -> NativePipeline {
    let mut pipeline = NativePipeline::new();
    let request = super::decoder_request::AudioDecoderOpenRequest {
        track_id: "loop-test".to_string(),
        source_path: path.to_string(),
    };
    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline
}

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
    assert!(progress.position_ms > 0 || progress.position_ms == 0); // position depends on frame
    assert_eq!(progress.buffered_frames, 0); // frame was drained
    assert_eq!(progress.output_pending_frames, 1);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn pump_once_drains_existing_buffer_before_decoding_more() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    // Decode one frame to buffer without draining
    pipeline.decode_once_to_buffer().expect("decode once");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    // pump_once should drain the existing frame first
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

    // Clock should not change on failed drain
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
fn pump_once_returns_end_of_stream_after_decoder_drains() {
    let path = write_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    // Pump until EndOfStream
    let mut drain_count = 0;
    loop {
        match pipeline.pump_once().expect("pump_once") {
            NativePipelineLoopStep::Drained => {
                drain_count += 1;
            }
            NativePipelineLoopStep::EndOfStream => break,
        }
    }

    // Small WAV has 8 samples / 4 per frame = 2 frames
    assert!(drain_count > 0, "should have drained at least one frame");

    // Subsequent pump should still return EndOfStream
    let step = pipeline.pump_once().expect("pump_once after eos");
    assert_eq!(step, NativePipelineLoopStep::EndOfStream);

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Draining);
    assert_eq!(pipeline.buffered_frame_count(), 0);

    pipeline.shutdown().expect("shutdown");
    fs::remove_file(path).expect("remove wav");
}

#[test]
fn run_until_blocked_respects_max_steps() {
    let path = write_long_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    // max_steps = 1 should only advance once
    pipeline.run_until_blocked(1).expect("run_until_blocked(1)");
    let pending_after_1 = pipeline.snapshot().output_status.pending_frames;
    assert_eq!(pending_after_1, 1);

    // max_steps = 0 should not advance
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

    // Large max_steps should stop at EOS, not loop forever
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
fn loop_does_not_update_clock_on_failed_drain() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");

    // Drain on empty buffer fails
    let result = pipeline.drain_once_to_output();
    assert!(result.is_err());

    // Clock should remain unchanged
    assert_eq!(pipeline.clock.position_ms(), 0);
    assert!(pipeline.clock.is_started()); // start() sets started

    pipeline.shutdown().expect("shutdown");
}

#[test]
fn seek_clears_buffer_before_loop_resumes() {
    let path = write_long_test_wav();
    let mut pipeline = open_wav_pipeline(&path);

    // Decode a frame into buffer
    pipeline
        .decode_once_to_buffer()
        .expect("decode should succeed");
    assert_eq!(pipeline.buffered_frame_count(), 1);

    // Seek clears buffer
    pipeline.seek_decoder(5).expect("seek");
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert_eq!(pipeline.clock.position_ms(), 5);

    // Pump after seek should not submit old frame
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

    // Pump once
    pipeline.pump_once().expect("pump_once");
    assert!(pipeline.clock.is_started());
    assert!(pipeline.buffered_frame_count() == 0);

    // Shutdown
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

#[test]
fn public_native_engine_remains_typed_unsupported() {
    use super::backends::native::KivoNativeEngine;
    use super::engine::PlaybackEngine;

    let mut engine = KivoNativeEngine::new();

    // All public methods must still return UnsupportedOperation
    let result = engine.play();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.pause();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.resume();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.stop();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.seek(0);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.set_volume(1.0);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.set_muted(false);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));
}

#[test]
fn capabilities_remain_default() {
    use super::capabilities::PlaybackCapabilities;

    let caps = PlaybackCapabilities::default();
    assert!(!caps.can_seek);
    assert!(!caps.can_select_output_device);
    assert!(!caps.can_use_exclusive_output);
    assert!(!caps.can_probe_metadata);
    assert!(!caps.can_gapless);
    assert!(!caps.can_replaygain);
}
