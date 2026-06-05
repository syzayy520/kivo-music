//! Tests for RingBuffer consume_frames API.

use super::buffer::RingBuffer;
use super::errors::RingBufferError;
use super::types::RingBufferFormat;

fn sample_format() -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    }
}

#[test]
fn consume_advances_read_pointer() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    rb.write_frames(&data).unwrap();

    // Consume 1 frame
    let consumed = rb.consume_frames(1).unwrap();
    assert_eq!(consumed, 1);
    assert_eq!(rb.available_frames(), 2);

    // Read remaining frames
    let mut out = vec![0u8; 16];
    let read = rb.read_frames_or_silence(&mut out).unwrap();
    assert_eq!(read, 2);
    // Should read frames 2 and 3
    assert_eq!(
        out,
        vec![9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
    );
}

#[test]
fn consume_zero_noop() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    rb.write_frames(&data).unwrap();

    let initial_available = rb.available_frames();
    let consumed = rb.consume_frames(0).unwrap();
    assert_eq!(consumed, 0);
    assert_eq!(rb.available_frames(), initial_available);
}

#[test]
fn consume_more_than_available_is_error() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    rb.write_frames(&data).unwrap();

    let result = rb.consume_frames(2);
    assert_eq!(
        result.unwrap_err(),
        RingBufferError::NotEnoughFrames {
            requested: 2,
            available: 1,
        }
    );
    // State should be unchanged
    assert_eq!(rb.available_frames(), 1);
}

#[test]
fn consume_after_peek_success_pattern() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    // Peek
    let mut peek_out = vec![0u8; 16];
    let peeked = rb.peek_frames(&mut peek_out).unwrap();
    assert_eq!(peeked, 2);
    assert_eq!(rb.available_frames(), 2);

    // Consume same amount
    let consumed = rb.consume_frames(peeked).unwrap();
    assert_eq!(consumed, 2);
    assert_eq!(rb.available_frames(), 0);

    // Subsequent read should return 0 frames
    let mut read_out = vec![0u8; 16];
    let read = rb.read_frames_or_silence(&mut read_out).unwrap();
    assert_eq!(read, 0);
}

#[test]
fn consume_updates_stats() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let initial_stats = rb.stats();
    rb.consume_frames(1).unwrap();

    let new_stats = rb.stats();
    assert_eq!(
        new_stats.total_frames_read,
        initial_stats.total_frames_read + 1
    );
}

#[test]
fn consume_empty_open_with_positive_frames_returns_not_enough_frames() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    // Buffer is open and empty
    assert_eq!(rb.available_frames(), 0);

    let result = rb.consume_frames(1);
    assert_eq!(
        result.unwrap_err(),
        RingBufferError::NotEnoughFrames {
            requested: 1,
            available: 0,
        }
    );
}

#[test]
fn consume_closed_buffer_returns_error() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    rb.close();

    let result = rb.consume_frames(1);
    assert_eq!(result.unwrap_err(), RingBufferError::Closed);
}

#[test]
fn consume_closed_buffer_with_data_works() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    rb.write_frames(&data).unwrap();
    rb.close();

    // Should be able to consume even when closed (data still readable)
    let consumed = rb.consume_frames(1).unwrap();
    assert_eq!(consumed, 1);
    assert_eq!(rb.available_frames(), 0);
}
