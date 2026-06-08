//! Buffer consumption update helper.
//!
//! Pure function that updates the buffer state field of a ThreadSnapshot.

use crate::playback::output_wasapi::output_thread::runtime::thread_snapshot::ThreadSnapshot;
use crate::playback::output_wasapi::output_thread::state::BufferConsumptionState;

/// Update the buffer consumption state field of a snapshot.
///
/// Pure function — no side effects beyond the mutation.
pub fn update_buffer_state(snapshot: &mut ThreadSnapshot, state: BufferConsumptionState) {
    snapshot.buffer_state = state;
}
