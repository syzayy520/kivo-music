//! Event contract tests.
//!
//! Tests for event type variants, default implementations, and basic properties.

use crate::playback::output_wasapi::output_thread::event::{
    FailureEvent, RenderEvent, ThreadEvent,
};
use crate::playback::output_wasapi::output_thread::state::{
    OutputThreadFailureKind, OutputThreadLifecycle,
};

#[test]
fn thread_event_variants_exist() {
    let state_changed = ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::NotStarted,
        to: OutputThreadLifecycle::Running,
    };
    let spawned = ThreadEvent::Spawned { thread_id: 1 };
    let joined = ThreadEvent::Joined;

    assert_ne!(state_changed, spawned);
    assert_ne!(spawned, joined);
}

#[test]
fn render_event_variants_exist() {
    let frames = RenderEvent::FramesRendered {
        frame_count: 1024,
        bytes_written: 4096,
    };
    let silence = RenderEvent::SilenceWritten { frame_count: 512 };
    let empty = RenderEvent::EmptyRender;

    assert_ne!(frames, silence);
    assert_ne!(silence, empty);
}

#[test]
fn failure_event_variants_exist() {
    let runtime = FailureEvent::RuntimeError {
        kind: OutputThreadFailureKind::Unknown,
        message: "test".to_string(),
    };
    let drain = FailureEvent::DrainError {
        message: "drain".to_string(),
    };
    let flush = FailureEvent::FlushError {
        message: "flush".to_string(),
    };

    assert_ne!(runtime, drain);
    assert_ne!(drain, flush);
}

#[test]
fn thread_event_default_is_state_changed() {
    let event = ThreadEvent::default();
    assert_eq!(
        event,
        ThreadEvent::StateChanged {
            from: OutputThreadLifecycle::default(),
            to: OutputThreadLifecycle::default(),
        }
    );
}

#[test]
fn render_event_default_is_empty_render() {
    let event = RenderEvent::default();
    assert_eq!(event, RenderEvent::EmptyRender);
}

#[test]
fn failure_event_default_is_runtime_error_unknown() {
    let event = FailureEvent::default();
    assert_eq!(
        event,
        FailureEvent::RuntimeError {
            kind: OutputThreadFailureKind::Unknown,
            message: String::new(),
        }
    );
}

#[test]
fn thread_event_clone() {
    let event = ThreadEvent::Spawned { thread_id: 42 };
    let cloned = event.clone();
    assert_eq!(event, cloned);
}

#[test]
fn render_event_clone() {
    let event = RenderEvent::FramesRendered {
        frame_count: 256,
        bytes_written: 1024,
    };
    let cloned = event.clone();
    assert_eq!(event, cloned);
}

#[test]
fn failure_event_clone() {
    let event = FailureEvent::RuntimeError {
        kind: OutputThreadFailureKind::Timeout,
        message: "timeout".to_string(),
    };
    let cloned = event.clone();
    assert_eq!(event, cloned);
}

#[test]
fn thread_event_debug() {
    let event = ThreadEvent::Joined;
    let debug = format!("{:?}", event);
    assert!(debug.contains("Joined"));
}

#[test]
fn render_event_debug() {
    let event = RenderEvent::SilenceWritten { frame_count: 100 };
    let debug = format!("{:?}", event);
    assert!(debug.contains("SilenceWritten"));
}

#[test]
fn failure_event_debug() {
    let event = FailureEvent::FlushError {
        message: "test".to_string(),
    };
    let debug = format!("{:?}", event);
    assert!(debug.contains("FlushError"));
}

#[test]
fn thread_event_equality() {
    let a = ThreadEvent::Joined;
    let b = ThreadEvent::Joined;
    let c = ThreadEvent::Spawned { thread_id: 1 };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn render_event_equality() {
    let a = RenderEvent::EmptyRender;
    let b = RenderEvent::EmptyRender;
    let c = RenderEvent::SilenceWritten { frame_count: 1 };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn failure_event_equality() {
    let a = FailureEvent::DrainError {
        message: "test".to_string(),
    };
    let b = FailureEvent::DrainError {
        message: "test".to_string(),
    };
    let c = FailureEvent::DrainError {
        message: "other".to_string(),
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn thread_event_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = ThreadEvent::Joined;
    let b = ThreadEvent::Joined;

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn render_event_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = RenderEvent::EmptyRender;
    let b = RenderEvent::EmptyRender;

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn failure_event_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = FailureEvent::FlushError {
        message: "test".to_string(),
    };
    let b = FailureEvent::FlushError {
        message: "test".to_string(),
    };

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn thread_event_state_changed_different_transitions() {
    let event_1 = ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::NotStarted,
        to: OutputThreadLifecycle::Running,
    };
    let event_2 = ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::Running,
        to: OutputThreadLifecycle::Stopped,
    };
    assert_ne!(event_1, event_2);
}

#[test]
fn render_event_frames_rendered_different_counts() {
    let event_1 = RenderEvent::FramesRendered {
        frame_count: 100,
        bytes_written: 400,
    };
    let event_2 = RenderEvent::FramesRendered {
        frame_count: 200,
        bytes_written: 800,
    };
    assert_ne!(event_1, event_2);
}

#[test]
fn failure_event_runtime_error_different_kinds() {
    let event_1 = FailureEvent::RuntimeError {
        kind: OutputThreadFailureKind::Timeout,
        message: "timeout".to_string(),
    };
    let event_2 = FailureEvent::RuntimeError {
        kind: OutputThreadFailureKind::RuntimeError,
        message: "timeout".to_string(),
    };
    assert_ne!(event_1, event_2);
}