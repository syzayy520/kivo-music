//! Thread loop skeleton.
//!
//! Device-free, IO-free thread loop that processes commands
//! through the in-memory driver and produces events/snapshots.

pub mod loop_result;
pub mod loop_state;
pub mod loop_step;

pub use loop_result::LoopResult;
pub use loop_state::LoopState;
pub use loop_step::execute_loop_step;
