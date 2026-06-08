//! Thread command type.
//!
//! Defines the primary command enum for controlling the output thread.

use super::drain_command::DrainCommand;
use super::shutdown_command::ShutdownCommand;

/// Commands that can be sent to the output thread.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreadCommand {
    /// Gracefully shutdown the thread.
    Shutdown(ShutdownCommand),
    /// Drain the buffer before stopping.
    Drain(DrainCommand),
    /// Flush the output buffer immediately.
    Flush,
    /// Set the output volume (0.0 to 1.0).
    SetVolume(u32),
    /// Pause the output.
    Pause,
    /// Resume the output after pause.
    Resume,
}

impl Default for ThreadCommand {
    fn default() -> Self {
        Self::Shutdown(ShutdownCommand::default())
    }
}
