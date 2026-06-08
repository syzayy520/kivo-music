//! Runtime snapshot update tests.
//!
//! Tests for lifecycle, render activity, and buffer state update helpers.

use crate::playback::output_wasapi::output_thread::runtime::snapshot_update::{
    update_buffer_state, update_lifecycle, update_render_activity,
};
use crate::playback::output_wasapi::output_thread::runtime::thread_snapshot::ThreadSnapshot;
use crate::playback::output_wasapi::output_thread::state::{
    BufferConsumptionState, OutputThreadLifecycle, RenderActivity,
};

// --- Lifecycle update tests ---

#[test]
fn update_lifecycle_changes_field() {
    let mut snap = ThreadSnapshot::default();
    assert_eq!(snap.lifecycle, OutputThreadLifecycle::NotStarted);
    update_lifecycle(&mut snap, OutputThreadLifecycle::Running);
    assert_eq!(snap.lifecycle, OutputThreadLifecycle::Running);
}

#[test]
fn update_lifecycle_preserves_other_fields() {
    let mut snap = ThreadSnapshot::default();
    snap.total_frames_submitted = 100;
    update_lifecycle(&mut snap, OutputThreadLifecycle::Draining);
    assert_eq!(snap.total_frames_submitted, 100);
}

#[test]
fn update_lifecycle_to_terminal() {
    let mut snap = ThreadSnapshot::default();
    update_lifecycle(&mut snap, OutputThreadLifecycle::Failed);
    assert!(snap.lifecycle.is_terminal());
}

// --- Render activity update tests ---

#[test]
fn update_render_activity_changes_field() {
    let mut snap = ThreadSnapshot::default();
    assert_eq!(snap.render_activity, RenderActivity::Idle);
    update_render_activity(&mut snap, RenderActivity::Rendering);
    assert_eq!(snap.render_activity, RenderActivity::Rendering);
}

#[test]
fn update_render_activity_preserves_other_fields() {
    let mut snap = ThreadSnapshot::default();
    snap.total_frames_rendered = 50;
    update_render_activity(&mut snap, RenderActivity::Silenced);
    assert_eq!(snap.total_frames_rendered, 50);
}

#[test]
fn update_render_activity_to_error() {
    let mut snap = ThreadSnapshot::default();
    update_render_activity(&mut snap, RenderActivity::Error);
    assert_eq!(snap.render_activity, RenderActivity::Error);
}

// --- Buffer state update tests ---

#[test]
fn update_buffer_state_changes_field() {
    let mut snap = ThreadSnapshot::default();
    assert_eq!(snap.buffer_state, BufferConsumptionState::Empty);
    update_buffer_state(&mut snap, BufferConsumptionState::Available);
    assert_eq!(snap.buffer_state, BufferConsumptionState::Available);
}

#[test]
fn update_buffer_state_preserves_other_fields() {
    let mut snap = ThreadSnapshot::default();
    snap.total_frames_submitted = 200;
    update_buffer_state(&mut snap, BufferConsumptionState::Full);
    assert_eq!(snap.total_frames_submitted, 200);
}

#[test]
fn update_buffer_state_to_underrun() {
    let mut snap = ThreadSnapshot::default();
    update_buffer_state(&mut snap, BufferConsumptionState::Underrun);
    assert_eq!(snap.buffer_state, BufferConsumptionState::Underrun);
}
