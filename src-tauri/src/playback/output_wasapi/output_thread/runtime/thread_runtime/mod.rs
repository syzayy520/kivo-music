//! Thread runtime skeleton.
//!
//! Device-free, IO-free types representing the runtime-level
//! management of a spawned output thread: handle tracking,
//! spawn-to-handle mapping, and join boundary.

pub mod runtime_handle;
pub mod runtime_join;
pub mod runtime_spawn;

pub use runtime_handle::{JoinState, RuntimeHandle};
pub use runtime_join::{
    complete_join, compute_join_outcome, mark_join_requested, JoinOutcome, JoinRejectReason,
};
pub use runtime_spawn::create_runtime_handle;
