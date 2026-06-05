//! Core tests for RingBuffer drain operations.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::ring_buffer::types::RingBufferFormat;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteReport;

use super::drain_once::drain_ring_buffer_once_with_writer;
use super::report::WasapiRingBufferDrainReport;

pub(super) fn sample_format() -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    }
}

pub(super) fn sample_write_report(frames: u32) -> WasapiRenderWriteReport {
    WasapiRenderWriteReport {
        frames_written: frames,
        bytes_written: frames * 8, // 2 channels * 4 bytes
        used_silent_flag: false,
        sample_rate_hz: 44100,
        channels: 2,
    }
}

#[test]
fn drain_zero_requested_noop() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;
    let writer_called = std::cell::Cell::new(false);

    let report = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 0, |frames, bytes| {
        writer_called.set(true);
        Ok(sample_write_report(frames))
    })
    .unwrap();

    assert_eq!(report.requested_frames, 0);
    assert_eq!(report.peeked_frames, 0);
    assert_eq!(report.consumed_frames, 0);
    assert_eq!(rb.available_frames(), 2);
    assert_eq!(pending, 2);
    assert!(!writer_called.get());
}

#[test]
fn drain_empty_ring_buffer_noop() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let mut pending = 0;
    let writer_called = std::cell::Cell::new(false);

    let report = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 5, |frames, bytes| {
        writer_called.set(true);
        Ok(sample_write_report(frames))
    })
    .unwrap();

    assert_eq!(report.requested_frames, 5);
    assert_eq!(report.peeked_frames, 0);
    assert_eq!(report.consumed_frames, 0);
    assert_eq!(rb.available_frames(), 0);
    assert_eq!(pending, 0);
    assert!(!writer_called.get());
}

#[test]
fn writer_success_consumes_and_decrements_pending() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;

    let report = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        Ok(sample_write_report(frames))
    })
    .unwrap();

    assert_eq!(report.requested_frames, 2);
    assert_eq!(report.peeked_frames, 2);
    assert_eq!(report.rendered_frames, 2);
    assert_eq!(report.consumed_frames, 2);
    assert_eq!(report.bytes_rendered, 16);
    assert_eq!(report.pending_before, 2);
    assert_eq!(report.pending_after, 0);
    assert_eq!(rb.available_frames(), 0);
    assert_eq!(pending, 0);
}

#[test]
fn writer_success_partial_available() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8]; // 1 frame
    rb.write_frames(&data).unwrap();

    let mut pending = 1;

    let report = drain_ring_buffer_once_with_writer(
        &mut rb,
        &mut pending,
        5, // requested 5, only 1 available
        |frames, bytes| Ok(sample_write_report(frames)),
    )
    .unwrap();

    assert_eq!(report.requested_frames, 5);
    assert_eq!(report.peeked_frames, 1);
    assert_eq!(report.rendered_frames, 1);
    assert_eq!(report.consumed_frames, 1);
    assert_eq!(report.bytes_rendered, 8);
    assert_eq!(rb.available_frames(), 0);
    assert_eq!(pending, 0);
}

#[test]
fn writer_receives_peeked_bytes_in_order() {
    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    // Write 2 frames with recognizable bytes
    let data = vec![
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        0xAA,
    ];
    rb.write_frames(&data).unwrap();

    let mut pending = 2;

    let report = drain_ring_buffer_once_with_writer(&mut rb, &mut pending, 2, |frames, bytes| {
        // Verify bytes are in correct order
        assert_eq!(bytes.len(), 16);
        assert_eq!(
            &bytes[..8],
            &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22]
        );
        assert_eq!(
            &bytes[8..],
            &[0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA]
        );
        Ok(sample_write_report(frames))
    })
    .unwrap();

    assert_eq!(report.consumed_frames, 2);
    assert_eq!(rb.available_frames(), 0);
    assert_eq!(pending, 0);
}
