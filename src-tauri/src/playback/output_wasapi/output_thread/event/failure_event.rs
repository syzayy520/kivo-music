//! Failure event type.
//!
//! Defines events representing failures in the output thread.

use crate::playback::output_wasapi::output_thread::state::OutputThreadFailureKind;

/// Events emitted on output thread failures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FailureEvent {
    /// A runtime error occurred during output.
    RuntimeError {
        /// Classification of the failure.
        kind: OutputThreadFailureKind,
        /// Human-readable error description.
        message: String,
    },
    /// An error occurred during drain.
    DrainError {
        /// Human-readable error description.
        message: String,
    },
    /// An error occurred during flush.
    FlushError {
        /// Human-readable error description.
        message: String,
    },
}

impl Default for FailureEvent {
    fn default() -> Self {
        Self::RuntimeError {
            kind: OutputThreadFailureKind::Unknown,
            message: String::new(),
        }
    }
}
