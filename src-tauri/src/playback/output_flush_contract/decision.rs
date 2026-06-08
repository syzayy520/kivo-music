use super::ordering::FlushOrdering;
use super::request::OutputFlushRequest;
use super::target::FlushTarget;

/// Decision returned by the contract layer after evaluating a flush request.
///
/// The orchestrator consumes this to either execute the flush or short-circuit.
#[derive(Clone, Debug)]
pub enum OutputFlushDecision {
    /// No flush is needed (e.g. already flushed, or idle state with no output).
    NoFlushNeeded,

    /// Flush is required. Contains the ordering and targets to execute.
    FlushRequired(OutputFlushProceed),

    /// Seek is rejected due to invalid state or policy.
    RejectSeek(OutputFlushRejectReason),

    /// Defer to WASAPI barrier (render-thread generation ack required).
    DeferToWasapiBarrier(OutputFlushWasiBarrier),
}

/// Details for a FlushRequired decision.
#[derive(Clone, Debug)]
pub struct OutputFlushProceed {
    pub request: OutputFlushRequest,
    pub target: FlushTarget,
    pub ordering: FlushOrdering,
}

/// Reasons a seek may be rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputFlushRejectReason {
    /// Output route is in a closed/failed state.
    RouteClosed,
    /// Output sink is not open.
    SinkNotOpen,
    /// Flush is not supported for this request/target combination.
    UnsupportedCombination,
}

/// Details for a WASAPI barrier deferral.
#[derive(Clone, Debug)]
pub struct OutputFlushWasiBarrier {
    /// The expected generation counter the render thread must reach.
    pub expected_generation: u64,
    /// The targets that must be flushed before the barrier completes.
    pub targets: Vec<FlushTarget>,
}

impl OutputFlushDecision {
    /// Convenience: returns true if the decision is FlushRequired.
    pub fn is_flush_required(&self) -> bool {
        matches!(self, Self::FlushRequired(_))
    }

    /// Convenience: returns true if the decision is RejectSeek.
    pub fn is_reject(&self) -> bool {
        matches!(self, Self::RejectSeek(_))
    }
}
