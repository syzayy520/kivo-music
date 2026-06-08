//! Shutdown command type.
//!
//! Defines the shutdown command variants for the output thread.

/// Shutdown command with different urgency levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShutdownCommand {
    /// Graceful shutdown: drain buffer, then stop.
    Graceful,
    /// Immediate shutdown: stop without draining.
    Immediate,
    /// Shutdown with timeout: graceful until timeout, then immediate.
    WithTimeout(u64),
}

impl Default for ShutdownCommand {
    fn default() -> Self {
        Self::Graceful
    }
}