use super::super::decision::{
    OutputFlushDecision, OutputFlushProceed, OutputFlushRejectReason, OutputFlushWasiBarrier,
};
use super::super::ordering::FlushOrdering;
use super::super::request::OutputFlushRequest;
use super::super::target::FlushTarget;

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
