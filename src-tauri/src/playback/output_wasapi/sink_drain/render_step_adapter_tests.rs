//! Tests for the render step adapter.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::sink::WasapiOutputSink;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteError;

use super::error::WasapiRingBufferDrainError;
use super::render_loop_error::WasapiRenderLoopStepError;
use super::render_loop_report::WasapiRenderLoopStepSkipReason;
use super::render_step_adapter::run_wasapi_render_step_adapter;
use super::render_step_adapter_error::WasapiRenderStepAdapterError;
use super::render_step_adapter_report::WasapiRenderStepAdapterConfig;
use super::tests_core::sample_format;
use super::tick_error::WasapiDrainTickError;
use super::tick_report::WasapiDrainTickSkipReason;

/// A: adapter disabled → delegates to step which returns Disabled.
#[test]
fn adapter_disabled_delegates_to_step_disabled() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 5;

    let config = WasapiRenderStepAdapterConfig {
        requested_frames: 10,
        enabled: false,
    };

    let report = run_wasapi_render_step_adapter(&mut sink, config).unwrap();

    assert!(!report.step_report.attempted_tick);
    assert_eq!(
        report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::Disabled
    );
    assert!(report.step_report.tick_report.is_none());
    assert_eq!(report.pending_before, 5);
    assert_eq!(report.pending_after, 5);
    assert_eq!(sink.runtime.pending_frames, 5);
}

/// B: adapter enabled + zero requested → step calls tick, tick skips with RequestedZero.
#[test]
fn adapter_enabled_zero_requested_delegates_to_step_and_tick_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 5;

    let config = WasapiRenderStepAdapterConfig {
        requested_frames: 0,
        enabled: true,
    };

    let report = run_wasapi_render_step_adapter(&mut sink, config).unwrap();

    assert!(report.step_report.attempted_tick);
    assert_eq!(
        report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report.step_report.tick_report.as_ref().unwrap();
    assert!(!tick_report.attempted);
    assert_eq!(
        tick_report.skipped_reason,
        WasapiDrainTickSkipReason::RequestedZero
    );
    assert_eq!(report.pending_before, 5);
    assert_eq!(report.pending_after, 5);
    assert_eq!(sink.runtime.pending_frames, 5);
}

/// C: adapter enabled + no pending → step calls tick, tick skips with NoPendingFrames.
#[test]
fn adapter_enabled_no_pending_delegates_to_step_and_tick_skips() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 0;

    let config = WasapiRenderStepAdapterConfig {
        requested_frames: 5,
        enabled: true,
    };

    let report = run_wasapi_render_step_adapter(&mut sink, config).unwrap();

    assert!(report.step_report.attempted_tick);
    assert_eq!(
        report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report.step_report.tick_report.as_ref().unwrap();
    assert!(!tick_report.attempted);
    assert_eq!(
        tick_report.skipped_reason,
        WasapiDrainTickSkipReason::NoPendingFrames
    );
    assert_eq!(report.pending_before, 0);
    assert_eq!(report.pending_after, 0);
}

/// D: adapter enabled + pending + no ring buffer → step attempts drain, returns no-op.
#[test]
fn adapter_enabled_pending_delegates_to_step() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 3;
    // ring_buffer is None → drain returns no-op report

    let config = WasapiRenderStepAdapterConfig {
        requested_frames: 5,
        enabled: true,
    };

    let report = run_wasapi_render_step_adapter(&mut sink, config).unwrap();

    assert!(report.step_report.attempted_tick);
    assert_eq!(
        report.step_report.skipped_reason,
        WasapiRenderLoopStepSkipReason::None
    );

    let tick_report = report.step_report.tick_report.as_ref().unwrap();
    assert!(tick_report.attempted);
    assert_eq!(tick_report.skipped_reason, WasapiDrainTickSkipReason::None);

    let drain_report = tick_report.drain_report.as_ref().unwrap();
    assert_eq!(drain_report.peeked_frames, 0);
    assert_eq!(drain_report.consumed_frames, 0);

    assert_eq!(report.pending_before, 3);
    assert_eq!(report.pending_after, 3);
    assert_eq!(sink.runtime.pending_frames, 3);
}

/// E: step error propagates, pending preserved, ring buffer not consumed.
#[test]
fn adapter_step_error_propagates() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 2;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![1u8; 16]; // 2 frames at block_align=8
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    let config = WasapiRenderStepAdapterConfig {
        requested_frames: 2,
        enabled: true,
    };

    let result = run_wasapi_render_step_adapter(&mut sink, config);

    let err = result.unwrap_err();
    assert_eq!(
        err,
        WasapiRenderStepAdapterError::Step(WasapiRenderLoopStepError::Tick(
            WasapiDrainTickError::Drain(WasapiRingBufferDrainError::RenderWrite(
                WasapiRenderWriteError::NotOpen
            ))
        ))
    );

    // pending preserved
    assert_eq!(sink.runtime.pending_frames, 2);
    // ring buffer data preserved
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 2);
}

/// F: no implicit drain — state untouched without explicit adapter call.
#[test]
fn adapter_does_not_auto_run() {
    let mut sink = WasapiOutputSink::new();
    sink.runtime.pending_frames = 4;

    let mut rb = RingBuffer::new(sample_format(), 10).unwrap();
    let data = vec![0xAA_u8; 32]; // 4 frames
    rb.write_frames(&data).unwrap();
    sink.ring_buffer = Some(rb);

    // After constructing sink with data + pending, nothing should have changed
    assert_eq!(sink.runtime.pending_frames, 4);
    assert_eq!(sink.ring_buffer.as_ref().unwrap().available_frames(), 4);

    // No implicit drain — state is untouched until explicit adapter call
}
