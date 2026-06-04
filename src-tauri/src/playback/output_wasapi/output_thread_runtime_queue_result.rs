use super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry;
use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;

/// Reason a queue entry was rejected.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueRejectReason {
    /// Queue is closed.
    Closed,
    /// Queue is at capacity.
    Full,
    /// Entry is invalid for the current runtime.
    InvalidForRuntime,
}

/// Result of accepting a queue entry.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueAcceptResult {
    /// The accepted entry.
    pub entry: OutputThreadRuntimeQueueEntry,
    /// The updated queue state.
    pub state: OutputThreadRuntimeQueueState,
}

/// Result of rejecting a queue entry.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueRejectResult {
    /// The reason for rejection.
    pub reason: OutputThreadRuntimeQueueRejectReason,
    /// The updated queue state.
    pub state: OutputThreadRuntimeQueueState,
}

/// Overall result of planning a queue intent.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueuePlanResult {
    /// The intent was accepted.
    Accepted(OutputThreadRuntimeQueueAcceptResult),
    /// The intent was rejected.
    Rejected(OutputThreadRuntimeQueueRejectResult),
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueuePlanResult {
    /// Whether the result is an acceptance.
    pub(crate) fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted(_))
    }

    /// Whether the result is a rejection.
    pub(crate) fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected(_))
    }

    /// Extract the resulting queue state.
    pub(crate) fn state(self) -> OutputThreadRuntimeQueueState {
        match self {
            Self::Accepted(r) => r.state,
            Self::Rejected(r) => r.state,
        }
    }
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueRejectReason {
    /// Whether this is a capacity-related error.
    pub(crate) fn is_capacity_error(self) -> bool {
        matches!(self, Self::Full)
    }

    /// Whether this is a lifecycle-related error (queue closed).
    pub(crate) fn is_lifecycle_error(self) -> bool {
        matches!(self, Self::Closed)
    }
}
