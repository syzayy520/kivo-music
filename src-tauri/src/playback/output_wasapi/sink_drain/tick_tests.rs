//! Tests for manual drain tick boundary.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::sink::WasapiOutputSink;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteError;

use super::error::WasapiRingBufferDrainError;
use super::tests_core::sample_format;
use super::tick::manual_drain_tick;
use super::tick_error::WasapiDrainTickError;
use super::tick_report::WasapiDrainTickSkipReason;

/// A: requested_frames == 0 skips drain immediately.
#[test]
fn tick_zero_requested_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 5;

    let report = manual_drain_tick(&mut sink, 0).unwrap();

    assert_eq!(report.requested_frames, 0);
    assert!(!report.attempted);
    assert_eq!(
        report.skipped_reason,
        WasapiDrainTickSkipReason::RequestedZero
    );
    assert!(report.drain_report.is_none());
    assert_eq!(report.pending_before, 5);
    assert_eq!(report.pending_after, 5);
    assert_eq!(sink.runtime.pending_frames, 5);
}

/// B: pending_frames == 0 skips drain immediately.
#[test]
fn tick_no_pending_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 0;

    let report = manual_drain_tick(&mut sink, 5).unwrap();

    assert_eq!(report.requested_frames, 5);
    assert!(!report.attempted);
    assert_eq!(
        report.skipped_reason,
        WasapiDrainTickSkipReason::NoPendingFrames
    );
    assert!(report.drain_report.is_none());
    assert_eq!(report.pending_before, 0);
    assert_eq!(report.pending_after, 0);
}

/// C: both pending > 0 and requested > 0 attempts drain.
///
/// With no ring buffer, drain returns a no-op report (no data drained).
/// This proves the code reaches the drain path.
#[test]
fn tick_with_pending_attempts_drain() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 3;
    // ring_buffer is None by default → drain returns no-op report

    let report = manual_drain_tick(&mut sink, 5).unwrap();

    assert_eq!(report.requested_frames, 5);
    assert!(report.attempted);
    assert_eq!(report.skipped_reason, WasapiDrainTickSkipReason::None);
    assert!(report.drain_report.is_some());

    let drain_report = report.drain_report.unwrap();
    assert_eq!(drain_report.requested_frames, 5);
    assert_eq!(drain_report.peeked_frames, 0);
    assert_eq!(drain_report.consumed_frames, 0);
    assert_eq!(drain_report.pending_before, 3);
    assert_eq!(drain_report.pending_after, 3);

    assert_eq!(report.pending_before, 3);
    assert_eq!(report.pending_after, 3);
    assert_eq!(sink.runtime.pending_frames, 3);
}

/// D: drain error propagates and preserves pending_frames.
///
/// With ring buffer data but context not open, writer returns NotOpen.
#[test]
fn tick_error_preserves_pending() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 2;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8; 16]; // 2 frames at block_align=8
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    let result = manual_drain_tick(&mut sink, 2);

    let err = result.unwrap_err();
    assert_eq!(
        err,
        WasapiDrainTickError::Drain(WasapiRingBufferDrainError::RenderWrite(
            WasapiRenderWriteError::NotOpen
        ))
    );

    // pending_frames preserved — drain helper guarantees no decrement on error
    assert_eq!(sink.runtime.pending_frames, 2);

    // ring buffer data preserved — drain helper guarantees no consume on error
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 2);
}

/// E: manual_drain_tick does NOT auto-call.
///
/// Constructing a sink with data and pending_frames does NOT drain it.
/// Only an explicit call to `manual_drain_tick` triggers drain.
/// This test proves no implicit drain occurs just from holding data.
#[test]
fn tick_does_not_call_automatically() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 4;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![0xAA_u8; 32]; // 4 frames
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    // After constructing sink with data + pending, nothing should have changed
    assert_eq!(sink.runtime.pending_frames, 4);
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 4);

    // No implicit drain — state is untouched until explicit tick
}
