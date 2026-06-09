//! Pump error.
//!
//! Errors that can occur during a runtime pump tick.

use std::fmt;

/// Errors from runtime pump execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PumpError {
    /// The loop state is terminal and cannot accept ticks.
    TerminalState,
    /// Sink dispatch failed during the tick.
    DispatchFailed { description: String },
    /// Internal pump error.
    Internal { description: String },
}

impl fmt::Display for PumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TerminalState => write!(f, "pump tick on terminal state"),
            Self::DispatchFailed { description } => {
                write!(f, "pump dispatch failed: {}", description)
            }
            Self::Internal { description } => write!(f, "pump internal error: {}", description),
        }
    }
}
