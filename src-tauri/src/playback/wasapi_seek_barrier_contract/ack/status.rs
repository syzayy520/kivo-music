/// Ack status classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiSeekBarrierAckStatus {
    /// Barrier completed for the matching generation.
    Completed,
    /// Barrier was rejected.
    Rejected,
    /// Barrier failed.
    Failed,
    /// Barrier is unsupported.
    Unsupported,
    /// Generation mismatch — stale ack.
    StaleGeneration,
}
