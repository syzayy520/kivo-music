//! Output thread event types.
//!
//! Pure data types representing events emitted by the output thread.
//! No behavior, no side effects.

pub mod failure_event;
pub mod render_event;
pub mod thread_event;

// Re-export primary types for convenience.
pub use failure_event::FailureEvent;
pub use render_event::RenderEvent;
pub use thread_event::ThreadEvent;
