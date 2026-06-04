/// Pure configuration for the runtime command queue.
///
/// No system queries, no allocations, no sync primitives.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueConfig {
    /// Maximum number of pending commands before the queue is full.
    pub max_pending_commands: u16,
    /// Whether to drop duplicate lifecycle commands (e.g., repeated Start).
    pub drop_duplicate_lifecycle_commands: bool,
    /// Whether a reset is allowed when the runtime is in Stopped state.
    pub allow_reset_when_stopped: bool,
}

impl Default for OutputThreadRuntimeQueueConfig {
    fn default() -> Self {
        Self {
            max_pending_commands: 32,
            drop_duplicate_lifecycle_commands: true,
            allow_reset_when_stopped: true,
        }
    }
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueConfig {
    /// Strict configuration with low capacity and conservative policies.
    pub(crate) fn strict() -> Self {
        Self {
            max_pending_commands: 8,
            drop_duplicate_lifecycle_commands: true,
            allow_reset_when_stopped: false,
        }
    }

    /// Permissive configuration with high capacity and relaxed policies.
    pub(crate) fn permissive() -> Self {
        Self {
            max_pending_commands: 64,
            drop_duplicate_lifecycle_commands: false,
            allow_reset_when_stopped: true,
        }
    }

    /// Whether this config has a finite capacity limit.
    pub(crate) fn has_capacity_limit(self) -> bool {
        self.max_pending_commands > 0
    }

    /// Whether the given pending count is within capacity.
    pub(crate) fn accepts_pending_count(self, pending_count: u16) -> bool {
        pending_count < self.max_pending_commands
    }
}
