//! Driver step type.
//!
//! Pure data enum representing a single step the driver can take.
//! No execution logic — only classification of possible actions.

/// A single step the output thread driver may take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverStep {
    /// Process the next command from the command queue.
    ProcessCommand,
    /// Render a cycle of audio frames.
    RenderCycle,
    /// Idle — no work to do, sleep briefly.
    Idle,
    /// Drain remaining frames before shutdown.
    Drain,
    /// Shut down the driver.
    Shutdown,
}
