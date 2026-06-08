//! Output thread runtime types.
//!
//! Pure data types representing the runtime lifecycle shell of the output thread.
//! ThreadHandle, ThreadSnapshot, and ThreadControl. No actual thread spawning.

pub mod driver;
pub mod queue;
pub mod state_machine;
pub mod thread_control;
pub mod thread_handle;
pub mod thread_snapshot;

// Re-export primary types for convenience.
pub use thread_control::ThreadControl;
pub use thread_handle::ThreadHandle;
pub use thread_snapshot::ThreadSnapshot;
