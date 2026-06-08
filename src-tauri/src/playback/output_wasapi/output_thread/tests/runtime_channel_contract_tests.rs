//! Channel boundary contract tests.
//!
//! Verifies that command/event channel types correctly wrap
//! mpsc channels and maintain type safety.

use std::sync::mpsc;

use crate::playback::output_wasapi::output_thread::command::{ShutdownCommand, ThreadCommand};
use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::channel::{
    command_channel, event_channel, CommandSender, EventSender,
};

// ── Command Channel Tests ──────────────────────────────────────────────

#[test]
fn command_channel_creates_valid_pair() {
    let (tx, _rx) = command_channel();
    // Sender should be cloneable
    let _tx2 = tx.clone();
}

#[test]
fn command_sender_send_delivers_command() {
    let (tx, rx) = command_channel();
    let cmd = ThreadCommand::Shutdown(ShutdownCommand::Immediate);
    tx.send(cmd).expect("send should succeed");
    let received = rx.try_recv().expect("should receive command");
    assert!(matches!(
        received,
        ThreadCommand::Shutdown(ShutdownCommand::Immediate)
    ));
}

#[test]
fn command_sender_send_returns_err_when_receiver_dropped() {
    let (tx, rx) = command_channel();
    drop(rx);
    let cmd = ThreadCommand::Shutdown(ShutdownCommand::Immediate);
    let result = tx.send(cmd);
    assert!(result.is_err());
}

#[test]
fn command_receiver_try_recv_returns_empty_when_no_commands() {
    let (_tx, rx) = command_channel();
    let result = rx.try_recv();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), mpsc::TryRecvError::Empty));
}

#[test]
fn command_receiver_try_recv_returns_disconnected_when_sender_dropped() {
    let (tx, rx) = command_channel();
    drop(tx);
    let result = rx.try_recv();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        mpsc::TryRecvError::Disconnected
    ));
}

#[test]
fn command_channel_preserves_fifo_order() {
    let (tx, rx) = command_channel();
    tx.send(ThreadCommand::Pause).expect("send 1");
    tx.send(ThreadCommand::Resume).expect("send 2");
    tx.send(ThreadCommand::Flush).expect("send 3");

    assert!(matches!(rx.try_recv().unwrap(), ThreadCommand::Pause));
    assert!(matches!(rx.try_recv().unwrap(), ThreadCommand::Resume));
    assert!(matches!(rx.try_recv().unwrap(), ThreadCommand::Flush));
}

// ── Event Channel Tests ────────────────────────────────────────────────

#[test]
fn event_channel_creates_valid_pair() {
    let (tx, _rx) = event_channel();
    let _tx2 = tx.clone();
}

#[test]
fn event_sender_send_delivers_event() {
    let (tx, rx) = event_channel();
    let event = ThreadEvent::Joined;
    tx.send(event).expect("send should succeed");
    let received = rx.try_recv().expect("should receive event");
    assert!(matches!(received, ThreadEvent::Joined));
}

#[test]
fn event_sender_send_returns_err_when_receiver_dropped() {
    let (tx, rx) = event_channel();
    drop(rx);
    let event = ThreadEvent::Joined;
    let result = tx.send(event);
    assert!(result.is_err());
}

#[test]
fn event_receiver_drain_available_returns_all_events() {
    let (tx, rx) = event_channel();
    tx.send(ThreadEvent::Joined).expect("send 1");
    tx.send(ThreadEvent::Joined).expect("send 2");
    tx.send(ThreadEvent::Joined).expect("send 3");

    let events = rx.drain_available();
    assert_eq!(events.len(), 3);
}

#[test]
fn event_receiver_drain_available_returns_empty_when_no_events() {
    let (_tx, rx) = event_channel();
    let events = rx.drain_available();
    assert!(events.is_empty());
}

#[test]
fn event_receiver_drain_available_is_idempotent() {
    let (tx, rx) = event_channel();
    tx.send(ThreadEvent::Joined).expect("send");

    let first = rx.drain_available();
    assert_eq!(first.len(), 1);

    let second = rx.drain_available();
    assert!(second.is_empty());
}

// ── Type Safety Tests ──────────────────────────────────────────────────

#[test]
fn command_sender_is_clone() {
    fn assert_clone<T: Clone>() {}
    assert_clone::<CommandSender>();
}

#[test]
fn event_sender_is_clone() {
    fn assert_clone<T: Clone>() {}
    assert_clone::<EventSender>();
}

#[test]
fn command_sender_is_debug() {
    let (tx, _rx) = command_channel();
    let _ = format!("{:?}", tx);
}

#[test]
fn event_sender_is_debug() {
    let (tx, _rx) = event_channel();
    let _ = format!("{:?}", tx);
}
