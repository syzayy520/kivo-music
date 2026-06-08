//! Event buffer type.
//!
//! Append-only buffer for thread events. No threads, no channels.

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;

/// Append-only buffer for events emitted during driver steps.
#[derive(Debug, Clone, Default)]
pub struct EventBuffer {
    /// Internal storage preserving insertion order.
    events: Vec<ThreadEvent>,
}

impl EventBuffer {
    /// Create a new empty event buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an event to the end of the buffer.
    pub fn push(&mut self, event: ThreadEvent) {
        self.events.push(event);
    }

    /// Get the number of events in the buffer.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Take all events from the buffer, leaving it empty.
    ///
    /// Preserves insertion order.
    pub(super) fn take_all(&mut self) -> Vec<ThreadEvent> {
        std::mem::take(&mut self.events)
    }
}
