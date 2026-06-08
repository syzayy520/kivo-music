//! State transition type.
//!
//! Represents a single lifecycle state transition request.
//! Pure data — no validation, no execution.

use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// A request to transition from one lifecycle state to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transition {
    /// The current state.
    pub from: OutputThreadLifecycle,
    /// The desired target state.
    pub to: OutputThreadLifecycle,
}

impl Transition {
    /// Create a new transition request.
    pub fn new(from: OutputThreadLifecycle, to: OutputThreadLifecycle) -> Self {
        Self { from, to }
    }

    /// Check if this is an identity transition (same source and target).
    pub fn is_identity(self) -> bool {
        self.from == self.to
    }
}
