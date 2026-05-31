use super::audio_buffer::AudioFrameBuffer;
use super::decoder::{AudioSampleFormat, AudioStreamInfo, DecodedAudioFrame};

fn frame(position_ms: u64) -> DecodedAudioFrame {
    DecodedAudioFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms,
        samples: vec![0.25, -0.25, 0.5, -0.5],
    }
}

#[test]
fn push_and_pop_follow_fifo_order() {
    let mut buffer = AudioFrameBuffer::new(4);
    assert!(buffer.push(frame(10)).is_none());
    assert!(buffer.push(frame(20)).is_none());

    assert_eq!(buffer.pop().map(|item| item.position_ms), Some(10));
    assert_eq!(buffer.pop().map(|item| item.position_ms), Some(20));
    assert!(buffer.pop().is_none());
}

#[test]
fn clear_removes_all_frames() {
    let mut buffer = AudioFrameBuffer::new(2);
    let _ = buffer.push(frame(1));
    let _ = buffer.push(frame(2));
    assert_eq!(buffer.len(), 2);

    buffer.clear();

    assert!(buffer.is_empty());
}

#[test]
fn push_drops_oldest_frame_when_capacity_is_exceeded() {
    let mut buffer = AudioFrameBuffer::new(2);
    let _ = buffer.push(frame(100));
    let _ = buffer.push(frame(200));
    let dropped = buffer.push(frame(300));

    assert_eq!(dropped.map(|item| item.position_ms), Some(100));
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.pop().map(|item| item.position_ms), Some(200));
    assert_eq!(buffer.pop().map(|item| item.position_ms), Some(300));
}

#[test]
fn capacity_zero_rejects_all_frames() {
    let mut buffer = AudioFrameBuffer::new(0);
    let dropped = buffer.push(frame(500));

    assert_eq!(dropped.map(|item| item.position_ms), Some(500));
    assert!(buffer.is_empty());
    assert_eq!(buffer.capacity_frames(), 0);
}
