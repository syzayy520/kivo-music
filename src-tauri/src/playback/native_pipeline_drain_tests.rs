use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::output::AudioOutputFrame;

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
}

fn write_test_wav() -> String {
    let path = std::env::temp_dir().join(format!(
        "kivo-native-pipeline-drain-{}-{}.wav",
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
        "kivo-native-pipeline-drain-long-{}-{}.wav",
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

fn test_frame(position_ms: u64) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms,
        samples: vec![0.0, 0.0, 0.0, 0.0],
    }
}

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
fn schedule_decode_step_pushes_frame_to_buffer() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = super::decoder_request::AudioDecoderOpenRequest {
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
    let request = super::decoder_request::AudioDecoderOpenRequest {
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

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn shutdown_clears_buffer() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let request = super::decoder_request::AudioDecoderOpenRequest {
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

    fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn stop_output_clears_buffer() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(0));
    assert_eq!(pipeline.buffered_frame_count(), 1);

    pipeline.stop_output().expect("stop output");
    assert_eq!(pipeline.buffered_frame_count(), 0);
}

#[test]
fn flush_output_clears_buffer() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("open null sink");

    pipeline.enqueue_decoded_frame(test_frame(0));
    assert_eq!(pipeline.buffered_frame_count(), 1);

    pipeline.flush_output().expect("flush output");
    assert_eq!(pipeline.buffered_frame_count(), 0);
}

#[test]
fn worker_load_decodes_buffer_drains_to_null_sink() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let track = super::types::PlaybackTrack {
        id: super::types::TrackId("worker-buffer-drain-1".to_string()),
        title: "Worker Buffer Drain".to_string(),
        artist: "Worker Artist".to_string(),
        source_path: path.clone(),
    };

    pipeline
        .handle_worker_command(
            &super::playback_worker_command::PlaybackWorkerCommand::Load { track },
        )
        .expect("worker load wav");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.last_error.is_none());
    assert_eq!(pipeline.buffered_frame_count(), 0);

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}
