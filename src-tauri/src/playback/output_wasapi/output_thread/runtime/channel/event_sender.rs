//! Event sender boundary.
//!
//! Wraps std::sync::mpsc::Sender<ThreadEvent> for type-safe
//! event emission from the output thread.

use std::sync::mpsc;

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;

/// Type-safe event sender wrapping mpsc::Sender.
#[derive(Debug, Clone)]
pub struct EventSender {
    inner: mpsc::Sender<ThreadEvent>,
}

impl EventSender {
    /// Create a new EventSender wrapping the given mpsc sender.
    pub fn new(sender: mpsc::Sender<ThreadEvent>) -> Self {
        Self { inner: sender }
    }

    /// Send an event from the output thread.
    ///
    /// Returns Ok(()) on success, Err if the receiver has been dropped.
    pub fn send(&self, event: ThreadEvent) -> Result<(), mpsc::SendError<ThreadEvent>> {
        self.inner.send(event)
    }
}

/// Create a bounded event channel pair.
pub fn event_channel() -> (EventSender, EventReceiver) {
    let (tx, rx) = mpsc::channel();
    (EventSender::new(tx), EventReceiver::new(rx))
}

// Import EventReceiver for the channel factory function.
use super::event_receiver::EventReceiver;
