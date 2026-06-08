//! Thread lifecycle state types.
//!
//! Defines the lifecycle states of the WASAPI output thread.

use serde::{Deserialize, Serialize};

/// Lifecycle states of the WASAPI output thread.
///
/// Transitions:
/// - NotStarted → Starting (thread spawn requested)
/// - Starting → Running (thread successfully started)
/// - Running → Draining (flush/stop requested, draining remaining frames)
/// - Running → Stopping (immediate stop requested)
/// - Draining → Stopping (drain complete or timeout)
/// - Stopping → Stopped (thread gracefully stopped)
/// - Starting → Failed (thread spawn failed)
/// - Running → Failed (runtime error)
/// - Draining → Failed (error during drain)
/// - Stopping → Failed (error during stop)
/// - Failed → NotStarted (reset after failure)
/// - Stopped → NotStarted (reset after stop)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OutputThreadLifecycle {
    /// Thread has not been spawned.
    #[default]
    NotStarted,
    /// Thread spawn requested, waiting for confirmation.
    Starting,
    /// Thread is running and processing frames.
    Running,
    /// Thread is draining remaining frames before stopping.
    Draining,
    /// Thread is stopping (immediate or after drain).
    Stopping,
    /// Thread has stopped gracefully.
    Stopped,
    /// Thread encountered an error.
    Failed,
}

impl OutputThreadLifecycle {
    /// Returns true if the thread is in an active state (Starting, Running, Draining).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Starting | Self::Running | Self::Draining)
    }

    /// Returns true if the thread is in a terminal state (Stopped, Failed).
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped | Self::Failed)
    }

    /// Returns true if the thread is in a transitional state (Starting, Stopping).
    pub fn is_transitional(self) -> bool {
        matches!(self, Self::Starting | Self::Stopping)
    }

    /// Returns true if the thread can accept new frames.
    pub fn can_accept_frames(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Returns true if the thread is draining (accepting no new frames).
    pub fn is_draining(self) -> bool {
        self == Self::Draining
    }
}
