//! Failure state types.
//!
//! Defines failure classification for the output thread.

use serde::{Deserialize, Serialize};

/// Failure kinds for the output thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputThreadFailureKind {
    /// Thread spawn failed.
    SpawnFailed,
    /// Runtime error during processing.
    RuntimeError,
    /// Error during drain phase.
    DrainError,
    /// Error during stop phase.
    StopError,
    /// Operation timed out.
    Timeout,
    /// Unknown or unspecified error.
    Unknown,
}

/// Statistics for failure tracking.
///
/// Tracks failure-related metrics for monitoring and debugging.
/// All counters are monotonic (never decremented).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureStats {
    /// Total errors encountered.
    pub error_count: u64,
    /// Last failure kind (if any).
    pub last_failure: Option<OutputThreadFailureKind>,
}

impl FailureStats {
    /// Create a new stats instance with all counters at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an error with a specific failure kind.
    pub fn record_error(&mut self, kind: OutputThreadFailureKind) {
        self.error_count = self.error_count.saturating_add(1);
        self.last_failure = Some(kind);
    }

    /// Record an error with unknown failure kind.
    pub fn record_unknown_error(&mut self) {
        self.record_error(OutputThreadFailureKind::Unknown);
    }

    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
