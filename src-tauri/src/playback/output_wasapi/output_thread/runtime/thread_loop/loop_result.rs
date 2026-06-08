//! Loop result type.
//!
//! Indicates the outcome of a loop step execution.

/// Result of a loop step execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopResult {
    /// Continue processing more steps.
    Continue,
    /// Stop the loop (terminal state reached).
    Stop,
    /// An error occurred.
    Error(String),
}
