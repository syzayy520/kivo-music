//! Loop state tracking.
//!
//! Tracks the current state of the thread loop including
//! lifecycle, idle count, and step count.

use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

/// Current state of the thread loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopState {
    /// Current lifecycle state.
    pub lifecycle: OutputThreadLifecycle,
    /// Number of consecutive idle steps.
    pub idle_count: u64,
    /// Total steps executed.
    pub step_count: u64,
}

impl LoopState {
    /// Create a new loop state starting from NotStarted.
    pub fn new() -> Self {
        Self {
            lifecycle: OutputThreadLifecycle::NotStarted,
            idle_count: 0,
            step_count: 0,
        }
    }

    /// Create a loop state with a specific lifecycle.
    pub fn with_lifecycle(lifecycle: OutputThreadLifecycle) -> Self {
        Self {
            lifecycle,
            idle_count: 0,
            step_count: 0,
        }
    }

    /// Update lifecycle and reset idle count on state change.
    pub fn transition_to(&mut self, new_lifecycle: OutputThreadLifecycle) {
        if self.lifecycle != new_lifecycle {
            self.idle_count = 0;
        }
        self.lifecycle = new_lifecycle;
        self.step_count += 1;
    }

    /// Record an idle step.
    pub fn record_idle(&mut self) {
        self.idle_count += 1;
        self.step_count += 1;
    }

    /// Check if the loop is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.lifecycle.is_terminal()
    }

    /// Check if the loop can accept commands.
    pub fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }
}

impl Default for LoopState {
    fn default() -> Self {
        Self::new()
    }
}
