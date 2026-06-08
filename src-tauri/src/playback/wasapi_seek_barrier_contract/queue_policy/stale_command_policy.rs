/// Policy for handling stale commands specifically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaleCommandPolicy {
    /// Reject stale commands with an error.
    Reject,
    /// Drop stale commands silently.
    DropSilently,
    /// Convert stale commands to a StaleGenerationAck.
    ConvertToStaleAck,
}
