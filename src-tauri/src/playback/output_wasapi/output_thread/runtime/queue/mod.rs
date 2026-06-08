//! Output thread runtime queue types.
//!
//! Pure data types representing command and event queues for the output thread.
//! Uses std::collections::VecDeque. No threads, no channels.

pub mod command_queue;
pub mod event_queue;

// Re-export primary types for convenience.
pub use command_queue::CommandQueue;
pub use event_queue::EventQueue;
