use super::output_thread_runtime_queue_bridge_projection::OutputThreadRuntimeQueueBridgeProjection;
use super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::runtime_queue::state::OutputThreadRuntimeQueueState;

/// Accepted bridge result.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeAccepted {
    pub entry: OutputThreadRuntimeQueueEntry,
    pub queue_state: OutputThreadRuntimeQueueState,
    pub projection: OutputThreadRuntimeQueueBridgeProjection,
}

/// Rejected bridge result.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeRejected {
    pub reason: OutputThreadRuntimeQueueRejectReason,
    pub queue_state: OutputThreadRuntimeQueueState,
    pub projection: OutputThreadRuntimeQueueBridgeProjection,
}

/// Result of bridge intent planning.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueBridgeResult {
    Accepted(OutputThreadRuntimeQueueBridgeAccepted),
    Rejected(OutputThreadRuntimeQueueBridgeRejected),
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueBridgeResult {
    pub(crate) fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted(_))
    }
    pub(crate) fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected(_))
    }
    pub(crate) fn queue_state(self) -> OutputThreadRuntimeQueueState {
        match self {
            Self::Accepted(a) => a.queue_state,
            Self::Rejected(r) => r.queue_state,
        }
    }
    pub(crate) fn projection(self) -> OutputThreadRuntimeQueueBridgeProjection {
        match self {
            Self::Accepted(a) => a.projection,
            Self::Rejected(r) => r.projection,
        }
    }
}
