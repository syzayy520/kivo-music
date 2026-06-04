use super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;
use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;

/// Immutable snapshot of the runtime command queue.
///
/// Pure snapshot — no real queue, no real resource.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueSnapshot {
    /// Queue configuration.
    pub config: OutputThreadRuntimeQueueConfig,
    /// Current queue state.
    pub state: OutputThreadRuntimeQueueState,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueSnapshot {
    /// Create a snapshot from explicit config and state.
    pub(crate) fn new(
        config: OutputThreadRuntimeQueueConfig,
        state: OutputThreadRuntimeQueueState,
    ) -> Self {
        Self { config, state }
    }

    /// Create an empty snapshot with the given config.
    pub(crate) fn empty(config: OutputThreadRuntimeQueueConfig) -> Self {
        Self {
            config,
            state: OutputThreadRuntimeQueueState::empty(),
        }
    }

    /// Current number of pending commands.
    pub(crate) fn pending_count(self) -> u16 {
        self.state.pending_count
    }

    /// Whether the queue has capacity for more commands.
    pub(crate) fn has_capacity(self) -> bool {
        self.state.pending_count < self.config.max_pending_commands
    }

    /// Whether the queue is closed.
    pub(crate) fn is_closed(self) -> bool {
        self.state.is_closed()
    }

    /// Whether the queue can accept a new command (open + has capacity).
    pub(crate) fn can_accept(self) -> bool {
        !self.state.closed && self.has_capacity()
    }
}
