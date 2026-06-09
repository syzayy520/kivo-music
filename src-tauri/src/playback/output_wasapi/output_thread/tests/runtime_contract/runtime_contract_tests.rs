//! Runtime contract tests.
//!
//! Tests for runtime type variants, default values, and basic properties.

use crate::playback::output_wasapi::output_thread::command::{ShutdownCommand, ThreadCommand};
use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::{
    ThreadControl, ThreadHandle, ThreadSnapshot,
};
use crate::playback::output_wasapi::output_thread::state::{
    BufferConsumptionState, FlushBarrierState, OutputThreadLifecycle, RenderActivity,
};

#[test]
fn thread_handle_default_is_zero() {
    let handle = ThreadHandle::default();
    assert_eq!(handle.id, 0);
    assert_eq!(handle.generation, 0);
}

#[test]
fn thread_handle_new() {
    let handle = ThreadHandle::new(42, 3);
    assert_eq!(handle.id, 42);
    assert_eq!(handle.generation, 3);
}

#[test]
fn thread_handle_is_valid() {
    let invalid = ThreadHandle::default();
    assert!(!invalid.is_valid());

    let valid = ThreadHandle::new(1, 0);
    assert!(valid.is_valid());
}

#[test]
fn thread_handle_equality() {
    let a = ThreadHandle::new(1, 2);
    let b = ThreadHandle::new(1, 2);
    let c = ThreadHandle::new(1, 3);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn thread_handle_clone() {
    let a = ThreadHandle::new(5, 1);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn thread_handle_debug() {
    let handle = ThreadHandle::new(1, 0);
    let debug = format!("{:?}", handle);
    assert!(debug.contains("ThreadHandle"));
}

#[test]
fn thread_handle_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = ThreadHandle::new(1, 2);
    let b = ThreadHandle::new(1, 2);

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn thread_snapshot_default_values() {
    let snapshot = ThreadSnapshot::default();
    assert_eq!(snapshot.handle, ThreadHandle::default());
    assert_eq!(snapshot.lifecycle, OutputThreadLifecycle::default());
    assert_eq!(snapshot.render_activity, RenderActivity::default());
    assert_eq!(snapshot.buffer_state, BufferConsumptionState::default());
    assert_eq!(snapshot.flush_state, FlushBarrierState::default());
    assert_eq!(snapshot.total_frames_submitted, 0);
    assert_eq!(snapshot.total_frames_rendered, 0);
}

#[test]
fn thread_snapshot_clone() {
    let a = ThreadSnapshot::default();
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn thread_snapshot_debug() {
    let snapshot = ThreadSnapshot::default();
    let debug = format!("{:?}", snapshot);
    assert!(debug.contains("ThreadSnapshot"));
}

#[test]
fn thread_snapshot_equality() {
    let a = ThreadSnapshot::default();
    let b = ThreadSnapshot::default();
    assert_eq!(a, b);
}

#[test]
fn thread_control_variants_exist() {
    let init = ThreadControl::Initialize {
        handle: ThreadHandle::default(),
        config: ThreadConfig::default(),
    };
    let cmd = ThreadControl::Command {
        handle: ThreadHandle::new(1, 0),
        command: ThreadCommand::Flush,
    };
    let snap = ThreadControl::SnapshotRequest {
        handle: ThreadHandle::new(1, 0),
    };

    assert_ne!(init, cmd);
    assert_ne!(cmd, snap);
}

#[test]
fn thread_control_default_is_initialize() {
    let control = ThreadControl::default();
    assert!(matches!(control, ThreadControl::Initialize { .. }));
}

#[test]
fn thread_control_clone() {
    let a = ThreadControl::Command {
        handle: ThreadHandle::new(1, 0),
        command: ThreadCommand::Shutdown(ShutdownCommand::Graceful),
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn thread_control_debug() {
    let control = ThreadControl::SnapshotRequest {
        handle: ThreadHandle::new(1, 0),
    };
    let debug = format!("{:?}", control);
    assert!(debug.contains("SnapshotRequest"));
}

#[test]
fn thread_control_equality() {
    let a = ThreadControl::Initialize {
        handle: ThreadHandle::new(1, 0),
        config: ThreadConfig::default(),
    };
    let b = ThreadControl::Initialize {
        handle: ThreadHandle::new(1, 0),
        config: ThreadConfig::default(),
    };
    assert_eq!(a, b);
}

#[test]
fn thread_snapshot_custom_values() {
    let snapshot = ThreadSnapshot {
        handle: ThreadHandle::new(7, 2),
        lifecycle: OutputThreadLifecycle::Running,
        render_activity: RenderActivity::Rendering,
        buffer_state: BufferConsumptionState::Available,
        flush_state: FlushBarrierState::NotFlushing,
        total_frames_submitted: 1000,
        total_frames_rendered: 999,
    };
    assert_eq!(snapshot.handle.id, 7);
    assert_eq!(snapshot.lifecycle, OutputThreadLifecycle::Running);
    assert_eq!(snapshot.total_frames_submitted, 1000);
}
