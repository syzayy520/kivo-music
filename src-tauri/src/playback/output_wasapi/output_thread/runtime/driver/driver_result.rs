//! Driver result type.
//!
//! Pure data enum representing the outcome of a driver step.
//! No behavior — only classification.

/// Outcome of a single driver step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverResult {
    /// The step completed successfully.
    Continue,
    /// The step completed and the driver should stop.
    Stop,
    /// The step encountered an error.
    Error,
    /// No work was available — driver is idle.
    Idle,
}
