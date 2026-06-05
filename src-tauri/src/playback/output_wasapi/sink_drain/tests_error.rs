//! Error handling tests for RingBuffer drain operations.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::wasapi_context::{
    WasapiRenderWriteError, WasapiRenderWriteReport,
};

use super::drain_once::drain_ring_buffer_once_with_writer;
use super::error::WasapiRingBufferDrainError;
use super::tests_core::{sample_format, sample_write_report};

#[test]
fn writer_error_does_not_consume() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;

    let result = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        Err(WasapiRenderWriteError::NotOpen)
    });

    assert_eq!(
        result.unwrap_err(),
        WasapiRingBufferDrainError::RenderWrite(WasapiRenderWriteError::NotOpen)
    );
    assert_eq!(rb.available_frames(), 2);
    assert_eq!(pending, 2);
}

#[test]
fn writer_frame_mismatch_does_not_consume() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;

    let result = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        // Writer reports wrong frame count
        Ok(sample_write_report(1))
    });

    assert_eq!(
        result.unwrap_err(),
        WasapiRingBufferDrainError::RenderedFrameMismatch {
            peeked: 2,
            rendered: 1,
        }
    );
    assert_eq!(rb.available_frames(), 2);
    assert_eq!(pending, 2);
}

#[test]
fn pending_underflow_does_not_consume() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 1; // Only 1 pending, but 2 frames available

    let result = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        Ok(sample_write_report(frames))
    });

    assert_eq!(
        result.unwrap_err(),
        WasapiRingBufferDrainError::PendingFrameUnderflow {
            pending: 1,
            consumed: 2,
        }
    );
    assert_eq!(rb.available_frames(), 2);
    assert_eq!(pending, 1);
}

#[test]
fn writer_byte_mismatch_does_not_consume() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;

    let result = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        // Writer reports correct frames but wrong bytes
        Ok(WasapiRenderWriteReport {
            frames_written: frames,
            bytes_written: 8, // Should be 16
            used_silent_flag: false,
            sample_rate_hz: 44100,
            channels: 2,
        })
    });

    assert_eq!(
        result.unwrap_err(),
        WasapiRingBufferDrainError::RenderedByteMismatch {
            expected: 16,
            rendered: 8,
        }
    );
    assert_eq!(rb.available_frames(), 2);
    assert_eq!(pending, 2);
}
