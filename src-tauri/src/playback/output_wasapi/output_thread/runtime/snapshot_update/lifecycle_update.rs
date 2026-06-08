//! Lifecycle update helper.
//!
//! Pure function that updates the lifecycle field of a ThreadSnapshot.

use crate::playback::output_wasapi::output_thread::runtime::thread_snapshot::ThreadSnapshot;
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Update the lifecycle field of a snapshot.
///
/// Pure function — no side effects beyond the mutation.
pub fn update_lifecycle(snapshot: &mut ThreadSnapshot, new_lifecycle: OutputThreadLifecycle) {
    snapshot.lifecycle = new_lifecycle;
}
