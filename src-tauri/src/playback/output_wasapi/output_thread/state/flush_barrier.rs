//! Flush barrier state types.
//!
//! Defines flush barrier tracking for the output thread.

use serde::{Deserialize, Serialize};

/// Flush barrier state of the output thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FlushBarrierState {
    /// No flush in progress.
    #[default]
    NotFlushing,
    /// Flush requested, waiting to start.
    Requested,
    /// Actively flushing remaining frames.
    Flushing,
    /// Flush completed successfully.
    Complete,
    /// Flush encountered an error.
    Error,
}

/// Statistics for flush barrier operations.
///
/// Tracks flush-related metrics for monitoring and debugging.
/// All counters are monotonic (never decremented).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlushBarrierStats {
    /// Total flush operations requested.
    pub flush_count: u64,
    /// Total flush errors encountered.
    pub flush_errors: u64,
}

impl FlushBarrierStats {
    /// Create a new stats instance with all counters at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a flush operation.
    pub fn record_flush(&mut self) {
        self.flush_count = self.flush_count.saturating_add(1);
    }

    /// Record a flush error.
    pub fn record_flush_error(&mut self) {
        self.flush_errors = self.flush_errors.saturating_add(1);
    }

    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
