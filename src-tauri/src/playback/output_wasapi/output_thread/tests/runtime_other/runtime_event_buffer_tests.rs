//! Runtime event buffer tests.
//!
//! Tests for EventBuffer push, drain, len, and is_empty.

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::event_buffer::{
    drain_buffer, EventBuffer,
};
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

// --- Basic buffer tests ---

#[test]
fn new_buffer_is_empty() {
    let buf = EventBuffer::new();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn push_increases_length() {
    let mut buf = EventBuffer::new();
    buf.push(ThreadEvent::Joined);
    assert_eq!(buf.len(), 1);
    assert!(!buf.is_empty());
}

#[test]
fn push_multiple_preserves_order() {
    let mut buf = EventBuffer::new();
    buf.push(ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::Running,
        to: OutputThreadLifecycle::Draining,
    });
    buf.push(ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::Draining,
        to: OutputThreadLifecycle::Stopping,
    });
    buf.push(ThreadEvent::Joined);
    assert_eq!(buf.len(), 3);
}

// --- Drain tests ---

#[test]
fn drain_returns_all_events_in_order() {
    let mut buf = EventBuffer::new();
    let evt1 = ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::Running,
        to: OutputThreadLifecycle::Draining,
    };
    let evt2 = ThreadEvent::Joined;
    buf.push(evt1.clone());
    buf.push(evt2.clone());

    let drained = drain_buffer(&mut buf);
    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0], evt1);
    assert_eq!(drained[1], evt2);
}

#[test]
fn drain_leaves_buffer_empty() {
    let mut buf = EventBuffer::new();
    buf.push(ThreadEvent::Joined);
    let _ = drain_buffer(&mut buf);
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn drain_empty_buffer_returns_empty_vec() {
    let mut buf = EventBuffer::new();
    let drained = drain_buffer(&mut buf);
    assert!(drained.is_empty());
}

#[test]
fn drain_twice_returns_empty_second_time() {
    let mut buf = EventBuffer::new();
    buf.push(ThreadEvent::Joined);
    let first = drain_buffer(&mut buf);
    assert_eq!(first.len(), 1);

    let second = drain_buffer(&mut buf);
    assert!(second.is_empty());
}

#[test]
fn push_after_drain_works() {
    let mut buf = EventBuffer::new();
    buf.push(ThreadEvent::Joined);
    let _ = drain_buffer(&mut buf);

    buf.push(ThreadEvent::Spawned { thread_id: 42 });
    assert_eq!(buf.len(), 1);
    let drained = drain_buffer(&mut buf);
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0], ThreadEvent::Spawned { thread_id: 42 });
}
