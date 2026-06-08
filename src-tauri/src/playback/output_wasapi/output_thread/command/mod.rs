//! Output thread command types.
//!
//! Pure data types representing commands that can be sent to the output thread.
//! No behavior, no side effects.

pub mod drain_command;
pub mod shutdown_command;
pub mod thread_command;

// Re-export primary types for convenience.
pub use drain_command::DrainCommand;
pub use shutdown_command::ShutdownCommand;
pub use thread_command::ThreadCommand;