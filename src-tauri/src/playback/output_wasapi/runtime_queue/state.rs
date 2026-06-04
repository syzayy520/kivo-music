/// Pure counter state for the runtime command queue.
///
/// Only stores counts — no command collection, no Vec, no allocations.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueState {
    /// Number of commands currently pending (not yet consumed).
    pub pending_count: u16,
    /// Total number of accepted commands.
    pub accepted_count: u64,
    /// Total number of rejected commands.
    pub rejected_count: u64,
    /// Last assigned sequence number.
    pub last_sequence: u64,
    /// Whether the queue is closed (no more commands accepted).
    pub closed: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueState {
    /// Create an empty queue state.
    pub(crate) fn empty() -> Self {
        Self {
            pending_count: 0,
            accepted_count: 0,
            rejected_count: 0,
            last_sequence: 0,
            closed: false,
        }
    }

    /// Create a queue state with explicit values.
    pub(crate) fn new(
        pending_count: u16,
        accepted_count: u64,
        rejected_count: u64,
        last_sequence: u64,
        closed: bool,
    ) -> Self {
        Self {
            pending_count,
            accepted_count,
            rejected_count,
            last_sequence,
            closed,
        }
    }

    /// Whether there are no pending commands.
    pub(crate) fn is_empty(self) -> bool {
        self.pending_count == 0
    }

    /// Whether the queue is closed.
    pub(crate) fn is_closed(self) -> bool {
        self.closed
    }

    /// Whether the queue can accept a new command.
    pub(crate) fn can_accept(self, max_pending_commands: u16) -> bool {
        !self.closed && self.pending_count < max_pending_commands
    }

    /// Return a new state after accepting a command.
    pub(crate) fn with_acceptance(self) -> Self {
        Self {
            pending_count: self.pending_count + 1,
            accepted_count: self.accepted_count + 1,
            last_sequence: self.last_sequence + 1,
            ..self
        }
    }

    /// Return a new state after rejecting a command.
    pub(crate) fn with_rejection(self) -> Self {
        Self {
            rejected_count: self.rejected_count + 1,
            ..self
        }
    }

    /// Return a new state after decrementing pending count (saturating at 0).
    pub(crate) fn with_pending_decrement(self) -> Self {
        Self {
            pending_count: self.pending_count.saturating_sub(1),
            ..self
        }
    }

    /// Return a new state marked as closed.
    pub(crate) fn closed(self) -> Self {
        Self {
            closed: true,
            ..self
        }
    }
}
