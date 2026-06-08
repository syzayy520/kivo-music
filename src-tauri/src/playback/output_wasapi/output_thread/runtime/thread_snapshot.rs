//! Thread snapshot type.
//!
//! Point-in-time snapshot of the output thread's state.

use crate::playback::output_wasapi::output_thread::state::{
    BufferConsumptionState, FlushBarrierState, OutputThreadLifecycle, RenderActivity,
};

use super::thread_handle::ThreadHandle;

/// Point-in-time snapshot of the output thread's state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadSnapshot {
    /// Handle of the thread this snapshot belongs to.
    pub handle: ThreadHandle,
    /// Current lifecycle state.
    pub lifecycle: OutputThreadLifecycle,
    /// Current render activity.
    pub render_activity: RenderActivity,
    /// Current buffer consumption state.
    pub buffer_state: BufferConsumptionState,
    /// Current flush barrier state.
    pub flush_state: FlushBarrierState,
    /// Total frames submitted since thread start.
    pub total_frames_submitted: u64,
    /// Total frames rendered since thread start.
    pub total_frames_rendered: u64,
}
