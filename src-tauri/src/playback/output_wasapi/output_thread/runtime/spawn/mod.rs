//! Output thread runtime spawn boundary types.
//!
//! Pure data types representing spawn requests and results for the output thread.
//! No actual thread spawning logic — only data definitions.

pub mod spawn_request;
pub mod spawn_result;

// Re-export primary types for convenience.
pub use spawn_request::SpawnRequest;
pub use spawn_result::{SpawnRejectReason, SpawnResult};
