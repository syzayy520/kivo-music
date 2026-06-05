//! Tests for RingBuffer peek_frames API.

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
fn peek_does_not_consume() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut out = vec![0u8; 16];
    let peeked = rb.peek_frames(&mut out).unwrap();
    assert_eq!(peeked, 2);
    assert_eq!(rb.available_frames(), 2);

    // Data should still be readable
    let mut read_out = vec![0u8; 16];
    let read = rb.read_frames_or_silence(&mut read_out).unwrap();
    assert_eq!(read, 2);
    assert_eq!(read_out, data);
}

#[test]
fn peek_empty_returns_zero_and_preserves_output() {
    let rb = RingBuffer::new(sample_format(), 10).unwrap();
    let mut out = vec![0xFFu8; 16];
    let peeked = rb.peek_frames(&mut out).unwrap();
    assert_eq!(peeked, 0);
    // Output should be unchanged (sentinel values preserved)
    assert_eq!(out, vec![0xFFu8; 16]);
}

#[test]
fn peek_wraparound_preserves_order() {
    let mut rb = RingBuffer::new(sample_format(), 4).unwrap();
    // Write 3 frames
    let data1 = vec![
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    rb.write_frames(&data1).unwrap();

    // Read 2 frames to advance read_frame
    let mut temp = vec![0u8; 16];
    rb.read_frames_or_silence(&mut temp).unwrap();

    // Write 3 more frames (will wrap around)
    let data2 = vec![
        25u8, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
        47, 48,
    ];
    rb.write_frames(&data2).unwrap();

    // Peek should return data in correct order
    let mut out = vec![0u8; 40]; // 5 frames
    let peeked = rb.peek_frames(&mut out).unwrap();
    assert_eq!(peeked, 4); // only 4 frames available

    // Verify order: frame 3 (from first write) then frames 4-6 (from second write)
    let expected = vec![
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47, 48,
    ];
    assert_eq!(out[..32], expected);
}

#[test]
fn peek_unaligned_output_is_error() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    rb.write_frames(&data).unwrap();

    let mut out = vec![0u8; 7]; // Not multiple of block_align (8)
    let result = rb.peek_frames(&mut out);
    assert_eq!(result.unwrap_err(), RingBufferError::FrameAlignment);
    // Output should be unchanged
    assert_eq!(out, vec![0u8; 7]);
}

#[test]
fn peek_closed_buffer_returns_zero() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    rb.close();

    let mut out = vec![0u8; 16];
    let peeked = rb.peek_frames(&mut out).unwrap();
    assert_eq!(peeked, 0);
}

#[test]
fn peek_partial_fill() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    rb.write_frames(&data).unwrap();

    // Request more frames than available
    let mut out = vec![0u8; 32]; // 4 frames requested, only 1 available
    let peeked = rb.peek_frames(&mut out).unwrap();
    assert_eq!(peeked, 1);
    // Only first 8 bytes should be copied
    assert_eq!(out[..8], data);
    // Rest should be unchanged (zeros from initialization)
    assert_eq!(out[8..], vec![0u8; 24]);
}
