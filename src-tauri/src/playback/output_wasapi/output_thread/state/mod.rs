//! Output thread state types.
//!
//! Pure data types with no behavior. Defines the lifecycle states,
//! render activity, buffer consumption, flush barrier, and failure
//! classification for the output thread.

pub mod buffer_consumption;
pub mod failure;
pub mod flush_barrier;
pub mod render_activity;
pub mod thread_lifecycle;

// Re-export primary types for convenience.
pub use buffer_consumption::{BufferConsumptionState, BufferConsumptionStats};
pub use failure::{FailureStats, OutputThreadFailureKind};
pub use flush_barrier::{FlushBarrierState, FlushBarrierStats};
pub use render_activity::{RenderActivity, RenderActivityStats};
pub use thread_lifecycle::OutputThreadLifecycle;
