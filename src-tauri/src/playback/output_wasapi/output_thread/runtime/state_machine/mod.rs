//! Output thread runtime state machine types.
//!
//! Pure data types representing state transitions for the output thread lifecycle.
//! No transition execution logic — only data definitions.

pub mod transition;
pub mod transition_error;
pub mod transition_result;

// Re-export primary types for convenience.
pub use transition::Transition;
pub use transition_error::TransitionError;
pub use transition_result::TransitionResult;
