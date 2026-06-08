//! Event receiver boundary.
//!
//! Wraps std::sync::mpsc::Receiver<ThreadEvent> for type-safe
//! event reception outside the output thread.

use std::sync::mpsc;

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;

/// Type-safe event receiver wrapping mpsc::Receiver.
#[derive(Debug)]
pub struct EventReceiver {
    inner: mpsc::Receiver<ThreadEvent>,
}

impl EventReceiver {
    /// Create a new EventReceiver wrapping the given mpsc receiver.
    pub fn new(receiver: mpsc::Receiver<ThreadEvent>) -> Self {
        Self { inner: receiver }
    }

    /// Try to receive an event without blocking.
    ///
    /// Returns Ok(event) if available, Err(TryRecvError) otherwise.
    pub fn try_recv(&self) -> Result<ThreadEvent, mpsc::TryRecvError> {
        self.inner.try_recv()
    }

    /// Drain all available events without blocking.
    ///
    /// Returns a Vec of all currently available events.
    pub fn drain_available(&self) -> Vec<ThreadEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.inner.try_recv() {
            events.push(event);
        }
        events
    }
}
