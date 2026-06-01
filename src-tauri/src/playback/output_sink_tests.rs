use super::errors::PlaybackError;
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

#[test]
fn open_is_typed_unsupported() {
    let mut sink = UnsupportedOutputSink::new();

    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("device-1".to_string());

    assert_unsupported(sink.open(&settings), "open");
    let status = sink.status();
    assert_eq!(status.active_device_id.as_deref(), Some("device-1"));
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
    assert!(!sink.status().is_active);
}

#[test]
fn close_resets_runtime_status() {
    let mut sink = UnsupportedOutputSink::new();
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("device-2".to_string());
    let _ = sink.open(&settings);

    let close_result = sink.close();

    assert!(close_result.is_ok());
    let status = sink.status();
    assert!(status.active_device_id.is_none());
    assert!(status.last_error.is_none());
    assert!(!status.is_active);
}
