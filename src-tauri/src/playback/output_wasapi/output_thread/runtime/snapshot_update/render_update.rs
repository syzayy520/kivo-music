//! Render activity update helper.
//!
//! Pure function that updates the render activity field of a ThreadSnapshot.

use crate::playback::output_wasapi::output_thread::runtime::thread_snapshot::ThreadSnapshot;
use crate::playback::output_wasapi::output_thread::state::RenderActivity;

/// Update the render activity field of a snapshot.
///
/// Pure function — no side effects beyond the mutation.
pub fn update_render_activity(snapshot: &mut ThreadSnapshot, activity: RenderActivity) {
    snapshot.render_activity = activity;
}
