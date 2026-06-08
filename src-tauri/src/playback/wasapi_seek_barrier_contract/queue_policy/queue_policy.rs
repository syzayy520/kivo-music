/// Queue policy for barrier execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiQueuePolicy {
    /// Drain all pending entries before the barrier completes.
    DrainBeforeBarrier,
    /// Drop entries from older generations.
    DropOlderGeneration,
    /// Reject stale commands (generation mismatch).
    RejectStaleCommands,
    /// Preserve only entries matching the current generation.
    PreserveCurrentGenerationOnly,
}

impl WasapiQueuePolicy {
    /// Whether stale commands cannot affect the current generation.
    pub fn isolates_current_generation(self) -> bool {
        matches!(
            self,
            Self::RejectStaleCommands | Self::PreserveCurrentGenerationOnly
        )
    }
}
