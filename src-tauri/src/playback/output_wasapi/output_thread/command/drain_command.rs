//! Drain command type.
//!
//! Defines the drain command variants for the output thread.

/// Drain command with different completion criteria.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrainCommand {
    /// Drain until buffer is empty.
    UntilEmpty,
    /// Drain with a timeout in milliseconds.
    WithTimeout(u64),
    /// Drain a specific number of frames.
    FrameCount(u64),
}

impl Default for DrainCommand {
    fn default() -> Self {
        Self::UntilEmpty
    }
}