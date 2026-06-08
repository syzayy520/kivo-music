//! Runtime state machine transition rules.
//!
//! Pure functions for validating lifecycle transitions and applying commands.

pub mod command_rules;
pub mod lifecycle_rules;

// Re-export primary functions for convenience.
pub use command_rules::apply_command;
pub use lifecycle_rules::validate_transition;
