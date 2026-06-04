use crate::playback::output_wasapi::ring_buffer::{RingBuffer, RingBufferError, RingBufferFormat};

fn test_format() -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4, // 2 channels * 2 bytes
    }
}

fn zero_frames(count: u32, block_align: usize) -> Vec<u8> {
    vec![0u8; count as usize * block_align]
}

#[test]
fn creates_buffer_with_valid_format() {
    let buf = RingBuffer::new(test_format(), 1024).unwrap();
    assert_eq!(buf.capacity_frames(), 1024);
    assert_eq!(buf.available_frames(), 0);
    assert_eq!(buf.free_frames(), 1024);
    assert!(buf.is_empty());
    assert!(!buf.is_full());
    assert!(!buf.is_closed());
}

#[test]
fn rejects_invalid_format() {
    let f = test_format();
    assert!(matches!(
        RingBuffer::new(
            RingBufferFormat {
                block_align: 0,
                ..f
            },
            1024
        ),
        Err(RingBufferError::InvalidFormat)
    ));
    assert!(matches!(
        RingBuffer::new(RingBufferFormat { channels: 0, ..f }, 1024),
        Err(RingBufferError::InvalidFormat)
    ));
    assert!(matches!(
        RingBuffer::new(
            RingBufferFormat {
                bits_per_sample: 0,
                ..f
            },
            1024
        ),
        Err(RingBufferError::InvalidFormat)
    ));
    assert!(matches!(
        RingBuffer::new(
            RingBufferFormat {
                sample_rate_hz: 0,
                ..f
            },
            1024
        ),
        Err(RingBufferError::InvalidFormat)
    ));
}

#[test]
fn rejects_zero_capacity() {
    assert!(matches!(
        RingBuffer::new(test_format(), 0),
        Err(RingBufferError::InvalidCapacity)
    ));
}

#[test]
fn write_frames_accepts_aligned_bytes() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let data = zero_frames(2, 4);
    let written = buf.write_frames(&data).unwrap();
    assert_eq!(written, 2);
    assert_eq!(buf.available_frames(), 2);
    assert_eq!(buf.stats().total_frames_written, 2);
}

#[test]
fn write_frames_rejects_unaligned_bytes() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let data = vec![0u8; 3]; // not aligned to block_align=4
    assert!(matches!(
        buf.write_frames(&data),
        Err(RingBufferError::FrameAlignment)
    ));
}

#[test]
fn read_frames_returns_written_data() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let data = zero_frames(2, 4);
    buf.write_frames(&data).unwrap();
    let mut out = vec![0u8; 2 * 4];
    let read = buf.read_frames_or_silence(&mut out).unwrap();
    assert_eq!(read, 2);
    assert_eq!(buf.available_frames(), 0);
    assert_eq!(buf.stats().total_frames_read, 2);
    assert!(out.iter().all(|&b| b == 0));
}

#[test]
fn read_empty_fills_silence() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let mut out = vec![0xFFu8; 4 * 4]; // pre-fill with non-zero
    let read = buf.read_frames_or_silence(&mut out).unwrap();
    assert_eq!(read, 0);
    assert!(out.iter().all(|&b| b == 0));
    assert_eq!(buf.stats().underrun_count, 1);
    assert_eq!(buf.stats().total_silence_frames_filled, 4);
}

#[test]
fn partial_read_fills_remaining_silence() {
    let mut buf = RingBuffer::new(test_format(), 8).unwrap();
    let data = zero_frames(2, 4);
    buf.write_frames(&data).unwrap();
    let mut out = vec![0xFFu8; 4 * 4];
    let read = buf.read_frames_or_silence(&mut out).unwrap();
    assert_eq!(read, 2);
    assert!(out.iter().all(|&b| b == 0));
    assert_eq!(buf.stats().underrun_count, 1);
    assert_eq!(buf.stats().total_silence_frames_filled, 2);
    assert_eq!(buf.stats().total_frames_read, 2);
}

#[test]
fn full_buffer_returns_would_block() {
    let mut buf = RingBuffer::new(test_format(), 2).unwrap();
    let data = zero_frames(2, 4);
    buf.write_frames(&data).unwrap();
    assert!(buf.is_full());
    assert!(matches!(
        buf.write_frames(&data),
        Err(RingBufferError::WouldBlock)
    ));
    assert_eq!(buf.stats().overrun_count, 1);
    // Existing data preserved
    assert_eq!(buf.available_frames(), 2);
}

#[test]
fn close_blocks_future_writes() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    buf.close();
    let data = zero_frames(1, 4);
    assert!(matches!(
        buf.write_frames(&data),
        Err(RingBufferError::Closed)
    ));
}

#[test]
fn closed_buffer_allows_drain_then_closed() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let data = zero_frames(2, 4);
    buf.write_frames(&data).unwrap();
    buf.close();
    // Drain allowed
    let mut out = vec![0u8; 2 * 4];
    let read = buf.read_frames_or_silence(&mut out).unwrap();
    assert_eq!(read, 2);
    // Now empty + closed
    let mut out2 = vec![0u8; 1 * 4];
    assert!(matches!(
        buf.read_frames_or_silence(&mut out2),
        Err(RingBufferError::Closed)
    ));
}

#[test]
fn reset_clears_buffer() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    let data = zero_frames(2, 4);
    buf.write_frames(&data).unwrap();
    buf.reset();
    assert_eq!(buf.available_frames(), 0);
    assert_eq!(buf.free_frames(), 4);
    assert!(buf.is_empty());
}

#[test]
fn reset_does_not_write_non_silent_data() {
    let mut buf = RingBuffer::new(test_format(), 4).unwrap();
    buf.write_frames(&zero_frames(2, 4)).unwrap();
    buf.reset();
    let mut out = vec![0xFFu8; 4 * 4];
    buf.read_frames_or_silence(&mut out).unwrap();
    assert!(out.iter().all(|&b| b == 0));
}
