use super::super::ack::{
    OutputFlushAck, OutputFlushCompletedDetails, OutputFlushFailedReason,
    OutputFlushRejectedReason, OutputFlushStaleGenerationDetails,
};
use super::super::decision::{
    OutputFlushDecision, OutputFlushProceed, OutputFlushRejectReason, OutputFlushWasiBarrier,
};
use super::super::failure::OutputFlushFailure;
use super::super::ordering::FlushOrdering;
use super::super::request::OutputFlushRequest;
use super::super::target::FlushTarget;

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
