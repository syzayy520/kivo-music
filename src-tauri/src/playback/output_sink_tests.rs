use super::errors::PlaybackError;
use super::native_output::KivoNativeOutputSink;
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};
use super::output_sink::UnsupportedOutputSink;
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

fn frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 0,
        samples: vec![0.0, 0.1],
    }
}

fn assert_unsupported(result: Result<OutputRuntimeStatus, PlaybackError>, op: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                format!("output sink operation is not implemented: {op}")
            );
        }
        Err(other) => panic!("expected unsupported operation, got {other}"),
        Ok(_) => panic!("expected unsupported operation, got success"),
    }
}

// --- UnsupportedOutputSink tests (unchanged) ---

#[test]
fn open_is_typed_unsupported() {
    let mut sink = UnsupportedOutputSink::new();

    assert_unsupported(sink.open(&OutputSettings::default()), "open");
}

#[test]
fn submit_frame_is_typed_unsupported() {
    let mut sink = UnsupportedOutputSink::new();

    assert_unsupported(sink.submit_frame(frame()), "submit_frame");
}

#[test]
fn pause_resume_flush_stop_are_typed_unsupported() {
    let mut sink = UnsupportedOutputSink::new();

    assert_unsupported(sink.pause(), "pause");
    assert_unsupported(sink.resume(), "resume");
    assert_unsupported(sink.flush(), "flush");
    assert_unsupported(sink.stop(), "stop");
}

// --- Null Sink boundary tests (KivoNativeOutputSink delegates to KivoNullOutputSink) ---

#[test]
fn null_sink_open_succeeds_and_sets_boundary_active() {
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("null sink open should succeed");

    assert!(status.is_open);
    assert!(status.is_active);
    assert!(status.last_error.is_none());
}

#[test]
fn null_sink_open_records_selected_device() {
    let mut sink = KivoNativeOutputSink::new();
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("null-device".to_string());

    let status = sink.open(&settings).expect("null sink open");

    assert_eq!(status.active_device_id.as_deref(), Some("null-device"));
}

#[test]
fn null_sink_submit_frame_increments_pending_counter() {
    let mut sink = KivoNativeOutputSink::new();
    sink.open(&OutputSettings::default()).expect("open");

    sink.submit_frame(frame()).expect("submit 1");
    sink.submit_frame(frame()).expect("submit 2");
    let status = sink.submit_frame(frame()).expect("submit 3");

    assert_eq!(status.pending_frames, 3);
    assert!(status.last_error.is_none());
}

#[test]
fn null_sink_stop_sets_inactive_and_clears_pending() {
    let mut sink = KivoNativeOutputSink::new();
    sink.open(&OutputSettings::default()).expect("open");
    sink.submit_frame(frame()).expect("submit");

    let status = sink.stop().expect("stop");

    assert!(!status.is_active);
    assert_eq!(status.pending_frames, 0);
}

#[test]
fn null_sink_flush_clears_pending_frames() {
    let mut sink = KivoNativeOutputSink::new();
    sink.open(&OutputSettings::default()).expect("open");
    sink.submit_frame(frame()).expect("submit");
    sink.submit_frame(frame()).expect("submit");

    let status = sink.flush().expect("flush");

    assert_eq!(status.pending_frames, 0);
}

#[test]
fn null_sink_close_resets_status() {
    let mut sink = KivoNativeOutputSink::new();
    sink.open(&OutputSettings::default()).expect("open");
    sink.submit_frame(frame()).expect("submit");

    sink.close().expect("close");

    let status = sink.status();
    assert!(!status.is_open);
    assert!(!status.is_active);
    assert_eq!(status.pending_frames, 0);
    assert!(status.last_error.is_none());
}

#[test]
fn null_sink_pause_and_resume_succeed_without_error() {
    let mut sink = KivoNativeOutputSink::new();
    sink.open(&OutputSettings::default()).expect("open");

    let paused = sink.pause().expect("pause");
    assert!(paused.last_error.is_none());

    let resumed = sink.resume().expect("resume");
    assert!(resumed.last_error.is_none());
}
