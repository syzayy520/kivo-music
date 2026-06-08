//! Transition error type.
//!
//! Pure data enum representing reasons a state transition may fail.
//! No behavior — only classification.

/// Reasons a state transition was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionError {
    /// The requested transition is not valid in the current state graph.
    InvalidTransition,
    /// Another transition is already in progress.
    ConcurrencyConflict,
    /// The transition timed out before completing.
    TimeoutExpired,
}
