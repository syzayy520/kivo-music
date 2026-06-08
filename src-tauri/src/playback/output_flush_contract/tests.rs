use super::ack::*;
use super::decision::*;
use super::failure::*;
use super::ordering::*;
use super::request::*;
use super::target::*;

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: OutputFlushRequest
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn request_seek_is_seek() {
    let req = OutputFlushRequest::Seek { position_ms: 1000 };
    assert!(req.is_seek());
}

#[test]
fn request_pause_is_not_seek() {
    let req = OutputFlushRequest::Pause;
    assert!(!req.is_seek());
}

#[test]
fn request_stop_is_not_seek() {
    let req = OutputFlushRequest::Stop;
    assert!(!req.is_seek());
}

#[test]
fn request_manual_is_not_seek() {
    let req = OutputFlushRequest::Manual;
    assert!(!req.is_seek());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: FlushTarget
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn target_pipeline_buffer_is_buffer_level() {
    let t = FlushTarget::PipelineBuffer;
    assert!(t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_output_sink_pending_frames_is_buffer_level() {
    let t = FlushTarget::OutputSinkPendingFrames;
    assert!(t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_production_output_route_input_is_neither() {
    let t = FlushTarget::ProductionOutputRouteInput;
    assert!(!t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_wasapi_ring_buffer_is_device_level() {
    let t = FlushTarget::WasapiRingBuffer;
    assert!(!t.is_buffer_level());
    assert!(t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_render_thread_queue_requires_render_thread() {
    let t = FlushTarget::RenderThreadQueue;
    assert!(!t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(t.requires_render_thread());
}

#[test]
fn target_device_render_buffer_is_device_level_and_requires_render_thread() {
    let t = FlushTarget::DeviceRenderBuffer;
    assert!(!t.is_buffer_level());
    assert!(t.is_device_level());
    assert!(t.requires_render_thread());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: FlushOrdering
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ordering_output_before_decoder_seek_flushes_before() {
    let o = FlushOrdering::OutputBeforeDecoderSeek;
    assert!(o.flushes_output_before_seek());
    assert!(!o.flushes_output_after_seek());
    assert!(!o.is_two_phase());
}

#[test]
fn ordering_decoder_seek_before_output_flushes_after() {
    let o = FlushOrdering::DecoderSeekBeforeOutput;
    assert!(!o.flushes_output_before_seek());
    assert!(o.flushes_output_after_seek());
    assert!(!o.is_two_phase());
}

#[test]
fn ordering_two_phase_barrier_flushes_both() {
    let o = FlushOrdering::TwoPhaseBarrier;
    assert!(o.flushes_output_before_seek());
    assert!(o.flushes_output_after_seek());
    assert!(o.is_two_phase());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: OutputFlushDecision
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn decision_no_flush_needed_is_not_flush_required() {
    let d = OutputFlushDecision::NoFlushNeeded;
    assert!(!d.is_flush_required());
    assert!(!d.is_reject());
}

#[test]
fn decision_flush_required_is_flush_required() {
    let d = OutputFlushDecision::FlushRequired(OutputFlushProceed {
        request: OutputFlushRequest::Manual,
        target: FlushTarget::PipelineBuffer,
        ordering: FlushOrdering::OutputBeforeDecoderSeek,
    });
    assert!(d.is_flush_required());
    assert!(!d.is_reject());
}

#[test]
fn decision_reject_seek_is_reject() {
    let d = OutputFlushDecision::RejectSeek(OutputFlushRejectReason::RouteClosed);
    assert!(!d.is_flush_required());
    assert!(d.is_reject());
}

#[test]
fn decision_defer_to_wasapi_barrier_is_not_flush_required() {
    let d = OutputFlushDecision::DeferToWasapiBarrier(OutputFlushWasiBarrier {
        expected_generation: 42,
        targets: vec![FlushTarget::WasapiRingBuffer],
    });
    assert!(!d.is_flush_required());
    assert!(!d.is_reject());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: OutputFlushAck
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ack_completed_is_success_with_generation() {
    let ack = OutputFlushAck::Completed(OutputFlushCompletedDetails {
        buffer_frames_cleared: 10,
        sink_discarded_frames: 5,
        generation: Some(42),
    });
    assert!(ack.is_success());
    assert_eq!(ack.generation(), Some(42));
}

#[test]
fn ack_completed_without_generation() {
    let ack = OutputFlushAck::Completed(OutputFlushCompletedDetails {
        buffer_frames_cleared: 10,
        sink_discarded_frames: 5,
        generation: None,
    });
    assert!(ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_rejected_is_not_success() {
    let ack = OutputFlushAck::Rejected(OutputFlushRejectedReason::RouteClosed);
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_failed_is_not_success() {
    let ack = OutputFlushAck::Failed(OutputFlushFailedReason::SinkFailure("error".into()));
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_unsupported_is_not_success() {
    let ack = OutputFlushAck::Unsupported;
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_stale_generation_is_not_success() {
    let ack = OutputFlushAck::StaleGeneration(OutputFlushStaleGenerationDetails {
        expected_generation: 10,
        actual_generation: 9,
    });
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: OutputFlushFailure
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn failure_route_closed_is_fatal() {
    let f = OutputFlushFailure::RouteClosed("closed".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}

#[test]
fn failure_sink_failure_is_data_loss() {
    let f = OutputFlushFailure::SinkFailure("error".into());
    assert!(f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_backpressure_is_retryable() {
    let f = OutputFlushFailure::Backpressure("full".into());
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_unsupported_target_is_fatal() {
    let f = OutputFlushFailure::UnsupportedTarget("unsupported".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}

#[test]
fn failure_stale_generation_is_retryable() {
    let f = OutputFlushFailure::StaleGeneration {
        expected_generation: 10,
        actual_generation: 9,
    };
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_render_thread_failure_is_retryable() {
    let f = OutputFlushFailure::RenderThreadFailure("timeout".into());
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_device_reset_failure_is_data_loss() {
    let f = OutputFlushFailure::DeviceResetFailure("reset failed".into());
    assert!(f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_unknown_is_fatal() {
    let f = OutputFlushFailure::Unknown("unknown".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: Contract lint anchor — all types are constructible and usable
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn contract_lint_anchor_all_types_constructible() {
    // Request variants
    let seek = OutputFlushRequest::Seek { position_ms: 1000 };
    let pause = OutputFlushRequest::Pause;
    let stop = OutputFlushRequest::Stop;
    let manual = OutputFlushRequest::Manual;

    // Target variants
    let pipeline = FlushTarget::PipelineBuffer;
    let sink_pending = FlushTarget::OutputSinkPendingFrames;
    let route_input = FlushTarget::ProductionOutputRouteInput;
    let wasapi_ring = FlushTarget::WasapiRingBuffer;
    let render_queue = FlushTarget::RenderThreadQueue;
    let device_render = FlushTarget::DeviceRenderBuffer;

    // Ordering variants
    let output_before = FlushOrdering::OutputBeforeDecoderSeek;
    let decoder_before = FlushOrdering::DecoderSeekBeforeOutput;
    let two_phase = FlushOrdering::TwoPhaseBarrier;

    // Decision variants
    let no_flush = OutputFlushDecision::NoFlushNeeded;
    let flush_req = OutputFlushDecision::FlushRequired(OutputFlushProceed {
        request: seek.clone(),
        target: pipeline.clone(),
        ordering: two_phase.clone(),
    });
    let reject = OutputFlushDecision::RejectSeek(OutputFlushRejectReason::RouteClosed);
    let defer = OutputFlushDecision::DeferToWasapiBarrier(OutputFlushWasiBarrier {
        expected_generation: 1,
        targets: vec![wasapi_ring.clone()],
    });

    // Ack variants
    let completed = OutputFlushAck::Completed(OutputFlushCompletedDetails {
        buffer_frames_cleared: 0,
        sink_discarded_frames: 0,
        generation: Some(0),
    });
    let rejected = OutputFlushAck::Rejected(OutputFlushRejectedReason::RouteClosed);
    let failed = OutputFlushAck::Failed(OutputFlushFailedReason::SinkFailure("test".into()));
    let unsupported = OutputFlushAck::Unsupported;
    let stale = OutputFlushAck::StaleGeneration(OutputFlushStaleGenerationDetails {
        expected_generation: 1,
        actual_generation: 0,
    });

    // Failure variants
    let route_closed = OutputFlushFailure::RouteClosed("test".into());
    let sink_failure = OutputFlushFailure::SinkFailure("test".into());
    let backpressure = OutputFlushFailure::Backpressure("test".into());
    let unsupported_target = OutputFlushFailure::UnsupportedTarget("test".into());
    let stale_gen = OutputFlushFailure::StaleGeneration {
        expected_generation: 1,
        actual_generation: 0,
    };
    let render_thread = OutputFlushFailure::RenderThreadFailure("test".into());
    let device_reset = OutputFlushFailure::DeviceResetFailure("test".into());
    let unknown = OutputFlushFailure::Unknown("test".into());

    // Use all values to prevent dead_code warnings
    let _ = (
        seek,
        pause,
        stop,
        manual,
        pipeline,
        sink_pending,
        route_input,
        wasapi_ring,
        render_queue,
        device_render,
        output_before,
        decoder_before,
        two_phase,
        no_flush,
        flush_req,
        reject,
        defer,
        completed,
        rejected,
        failed,
        unsupported,
        stale,
        route_closed,
        sink_failure,
        backpressure,
        unsupported_target,
        stale_gen,
        render_thread,
        device_reset,
        unknown,
    );
}
