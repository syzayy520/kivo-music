//! Runtime queue contract tests.
//!
//! Tests for CommandQueue and EventQueue: push/pop, peek, len, drain.

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::queue::{CommandQueue, EventQueue};
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

// --- CommandQueue tests ---

#[test]
fn command_queue_default_is_empty() {
    let queue = CommandQueue::default();
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn command_queue_push_pop_fifo() {
    let mut queue = CommandQueue::new();
    queue.push(ThreadCommand::Flush);
    queue.push(ThreadCommand::Pause);

    assert_eq!(queue.len(), 2);
    assert_eq!(queue.pop(), Some(ThreadCommand::Flush));
    assert_eq!(queue.pop(), Some(ThreadCommand::Pause));
    assert_eq!(queue.pop(), None);
}

#[test]
fn command_queue_peek() {
    let mut queue = CommandQueue::new();
    assert_eq!(queue.peek(), None);

    queue.push(ThreadCommand::Flush);
    assert_eq!(queue.peek(), Some(&ThreadCommand::Flush));
    assert_eq!(queue.len(), 1); // peek does not remove
}

#[test]
fn command_queue_clear() {
    let mut queue = CommandQueue::new();
    queue.push(ThreadCommand::Flush);
    queue.push(ThreadCommand::Pause);
    queue.clear();

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn command_queue_drain() {
    let mut queue = CommandQueue::new();
    queue.push(ThreadCommand::Flush);
    queue.push(ThreadCommand::Pause);
    queue.push(ThreadCommand::Resume);

    let drained = queue.drain();
    assert_eq!(drained.len(), 3);
    assert!(queue.is_empty());
}

#[test]
fn command_queue_clone() {
    let mut a = CommandQueue::new();
    a.push(ThreadCommand::Flush);
    let b = a.clone();
    assert_eq!(a.len(), b.len());
}

#[test]
fn command_queue_debug() {
    let queue = CommandQueue::new();
    let debug = format!("{:?}", queue);
    assert!(debug.contains("CommandQueue"));
}

// --- EventQueue tests ---

#[test]
fn event_queue_default_is_empty() {
    let queue = EventQueue::default();
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn event_queue_push_pop_fifo() {
    let mut queue = EventQueue::new();
    queue.push(ThreadEvent::Joined);
    queue.push(ThreadEvent::Spawned { thread_id: 1 });

    assert_eq!(queue.len(), 2);
    assert_eq!(queue.pop(), Some(ThreadEvent::Joined));
    assert_eq!(queue.pop(), Some(ThreadEvent::Spawned { thread_id: 1 }));
    assert_eq!(queue.pop(), None);
}

#[test]
fn event_queue_peek() {
    let mut queue = EventQueue::new();
    assert_eq!(queue.peek(), None);

    queue.push(ThreadEvent::Joined);
    assert_eq!(queue.peek(), Some(&ThreadEvent::Joined));
    assert_eq!(queue.len(), 1);
}

#[test]
fn event_queue_clear() {
    let mut queue = EventQueue::new();
    queue.push(ThreadEvent::Joined);
    queue.push(ThreadEvent::Spawned { thread_id: 1 });
    queue.clear();

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn event_queue_drain() {
    let mut queue = EventQueue::new();
    queue.push(ThreadEvent::Joined);
    queue.push(ThreadEvent::Spawned { thread_id: 1 });
    queue.push(ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::NotStarted,
        to: OutputThreadLifecycle::Running,
    });

    let drained = queue.drain();
    assert_eq!(drained.len(), 3);
    assert!(queue.is_empty());
}

#[test]
fn event_queue_clone() {
    let mut a = EventQueue::new();
    a.push(ThreadEvent::Joined);
    let b = a.clone();
    assert_eq!(a.len(), b.len());
}

#[test]
fn event_queue_debug() {
    let queue = EventQueue::new();
    let debug = format!("{:?}", queue);
    assert!(debug.contains("EventQueue"));
}

#[test]
fn command_queue_multiple_operations() {
    let mut queue = CommandQueue::new();
    queue.push(ThreadCommand::Flush);
    queue.push(ThreadCommand::Pause);
    assert_eq!(queue.pop(), Some(ThreadCommand::Flush));

    queue.push(ThreadCommand::Resume);
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.peek(), Some(&ThreadCommand::Pause));

    let drained = queue.drain();
    assert_eq!(drained.len(), 2);
}

#[test]
fn event_queue_multiple_operations() {
    let mut queue = EventQueue::new();
    queue.push(ThreadEvent::Joined);
    queue.push(ThreadEvent::Spawned { thread_id: 1 });
    assert_eq!(queue.pop(), Some(ThreadEvent::Joined));

    queue.push(ThreadEvent::StateChanged {
        from: OutputThreadLifecycle::Running,
        to: OutputThreadLifecycle::Stopped,
    });
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.peek(), Some(&ThreadEvent::Spawned { thread_id: 1 }));
}
