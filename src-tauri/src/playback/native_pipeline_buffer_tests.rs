use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::native_pipeline_buffer::NativePipelineBuffer;
use super::output::AudioOutputFrame;

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
fn buffer_starts_empty() {
    let buffer = NativePipelineBuffer::new();
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
}

#[test]
fn buffer_push_increases_length() {
    let mut buffer = NativePipelineBuffer::new();
    buffer.enqueue_frame(test_frame(0));
    assert_eq!(buffer.len(), 1);
    assert!(!buffer.is_empty());

    buffer.enqueue_frame(test_frame(100));
    assert_eq!(buffer.len(), 2);
}

#[test]
fn buffer_pop_follows_fifo_order() {
    let mut buffer = NativePipelineBuffer::new();
    buffer.enqueue_frame(test_frame(0));
    buffer.enqueue_frame(test_frame(100));
    buffer.enqueue_frame(test_frame(200));

    let first = buffer.drain_next_frame().expect("should have first frame");
    assert_eq!(first.position_ms, 0);

    let second = buffer.drain_next_frame().expect("should have second frame");
    assert_eq!(second.position_ms, 100);

    let third = buffer.drain_next_frame().expect("should have third frame");
    assert_eq!(third.position_ms, 200);

    assert!(buffer.is_empty());
}

#[test]
fn buffer_pop_returns_none_when_empty() {
    let mut buffer = NativePipelineBuffer::new();
    assert!(buffer.drain_next_frame().is_none());
}

#[test]
fn buffer_clear_removes_all_frames() {
    let mut buffer = NativePipelineBuffer::new();
    buffer.enqueue_frame(test_frame(0));
    buffer.enqueue_frame(test_frame(100));
    buffer.enqueue_frame(test_frame(200));

    buffer.clear();
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert!(buffer.drain_next_frame().is_none());
}
