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
