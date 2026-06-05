//! Tests for the output-thread adapter slot.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::sink::WasapiOutputSink;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteError;

use super::error::WasapiRingBufferDrainError;
use super::output_thread_adapter_slot::run_wasapi_output_thread_adapter_slot;
use super::output_thread_adapter_slot_error::WasapiOutputThreadAdapterSlotError;
use super::output_thread_adapter_slot_report::WasapiOutputThreadAdapterSlotConfig;
use super::render_loop_error::WasapiRenderLoopStepError;
use super::render_loop_report::WasapiRenderLoopStepSkipReason;
use super::render_step_adapter_error::WasapiRenderStepAdapterError;
use super::tests_core::sample_format;
use super::tick_error::WasapiDrainTickError;
use super::tick_report::WasapiDrainTickSkipReason;

/// A: slot disabled → delegates to adapter which returns Disabled.
#[test]
fn slot_disabled_delegates_to_adapter_disabled() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 5;

    let config = WasapiOutputThreadAdapterSlotConfig {
        requested_frames: 10,
        enabled: false,
    };

    let report = run_wasapi_output_thread_adapter_slot(&mut sink, config).unwrap();

    assert!(!report.adapter_report.step_report.attempted_tick);
    assert_eq!(
        report.adapter_report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::Disabled
    );
    assert!(report.adapter_report.step_report.tick_report.is_none());
    assert_eq!(report.pending_before, 5);
    assert_eq!(report.pending_after, 5);
    assert_eq!(sink.runtime.pending_frames, 5);
}

/// B: slot enabled + zero requested → adapter calls step, tick skips with RequestedZero.
#[test]
fn slot_enabled_zero_requested_delegates_to_adapter_and_tick_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 5;

    let config = WasapiOutputThreadAdapterSlotConfig {
        requested_frames: 0,
        enabled: true,
    };

    let report = run_wasapi_output_thread_adapter_slot(&mut sink, config).unwrap();

    assert!(report.adapter_report.step_report.attempted_tick);
    assert_eq!(
        report.adapter_report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report
        .adapter_report
        .step_report
        .tick_report
        .as_ref()
        .unwrap();
    assert!(!tick_report.attempted);
    assert_eq!(
        tick_report.skipped_reason,
        WasapiDrainTickSkipReason::RequestedZero
    );
    assert_eq!(report.pending_before, 5);
    assert_eq!(report.pending_after, 5);
    assert_eq!(sink.runtime.pending_frames, 5);
}

/// C: slot enabled + no pending → adapter calls step, tick skips with NoPendingFrames.
#[test]
fn slot_enabled_no_pending_delegates_to_adapter_and_tick_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 0;

    let config = WasapiOutputThreadAdapterSlotConfig {
        requested_frames: 5,
        enabled: true,
    };

    let report = run_wasapi_output_thread_adapter_slot(&mut sink, config).unwrap();

    assert!(report.adapter_report.step_report.attempted_tick);
    assert_eq!(
        report.adapter_report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report
        .adapter_report
        .step_report
        .tick_report
        .as_ref()
        .unwrap();
    assert!(!tick_report.attempted);
    assert_eq!(
        tick_report.skipped_reason,
        WasapiDrainTickSkipReason::NoPendingFrames
    );
    assert_eq!(report.pending_before, 0);
    assert_eq!(report.pending_after, 0);
}

/// D: slot enabled + pending + no ring buffer → adapter attempts drain, returns no-op.
#[test]
fn slot_enabled_pending_delegates_to_adapter() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 3;
    // ring_buffer is None → drain returns no-op report

    let config = WasapiOutputThreadAdapterSlotConfig {
        requested_frames: 5,
        enabled: true,
    };

    let report = run_wasapi_output_thread_adapter_slot(&mut sink, config).unwrap();

    assert!(report.adapter_report.step_report.attempted_tick);
    assert_eq!(
        report.adapter_report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report
        .adapter_report
        .step_report
        .tick_report
        .as_ref()
        .unwrap();
    assert!(tick_report.attempted);
    assert_eq!(tick_report.skipped_reason, WasapiDrainTickSkipReason::None);

    let drain_report = tick_report.drain_report.as_ref().unwrap();
    assert_eq!(drain_report.peeked_frames, 0);
    assert_eq!(drain_report.consumed_frames, 0);

    assert_eq!(report.pending_before, 3);
    assert_eq!(report.pending_after, 3);
    assert_eq!(sink.runtime.pending_frames, 3);
}

/// E: adapter error propagates, pending preserved, ring buffer not consumed.
#[test]
fn slot_adapter_error_propagates() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 2;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8; 16]; // 2 frames at block_align=8
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    let config = WasapiOutputThreadAdapterSlotConfig {
        requested_frames: 2,
        enabled: true,
    };

    let result = run_wasapi_output_thread_adapter_slot(&mut sink, config);

    let err = result.unwrap_err();
    assert_eq!(
        err,
        WasapiOutputThreadAdapterSlotError::Adapter(WasapiRenderStepAdapterError::Step(
            WasapiRenderLoopStepError::Tick(WasapiDrainTickError::Drain(
                WasapiRingBufferDrainError::RenderWrite(WasapiRenderWriteError::NotOpen)
            ))
        ))
    );

    // pending preserved
    assert_eq!(sink.runtime.pending_frames, 2);
    // ring buffer data preserved
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 2);
}

/// F: no implicit drain — state untouched without explicit slot call.
#[test]
fn slot_does_not_auto_run() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 4;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![0xAA_u8; 32]; // 4 frames
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    // After constructing sink with data + pending, nothing should have changed
    assert_eq!(sink.runtime.pending_frames, 4);
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 4);

    // No implicit drain — state is untouched until explicit slot call
}
