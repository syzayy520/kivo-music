//! Shutdown command type.
//!
//! Defines the shutdown command variants for the output thread.

/// Shutdown command with different urgency levels.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ShutdownCommand {
    /// Graceful shutdown: drain buffer, then stop.
    #[default]
    Graceful,
    /// Immediate shutdown: stop without draining.
    Immediate,
    /// Shutdown with timeout: graceful until timeout, then immediate.
    WithTimeout(u64),
}
