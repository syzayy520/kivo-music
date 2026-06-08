//! Event queue type.
//!
//! FIFO queue for ThreadEvent. Uses VecDeque internally.
//! No threads, no channels, no side effects.

use std::collections::VecDeque;

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;

/// FIFO event queue for the output thread.
#[derive(Debug, Clone, Default)]
pub struct EventQueue {
    /// Internal buffer.
    buffer: VecDeque<ThreadEvent>,
}

impl EventQueue {
    /// Create a new empty event queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an event to the back of the queue.
    pub fn push(&mut self, event: ThreadEvent) {
        self.buffer.push_back(event);
    }

    /// Pop an event from the front of the queue.
    pub fn pop(&mut self) -> Option<ThreadEvent> {
        self.buffer.pop_front()
    }

    /// Peek at the front event without removing it.
    pub fn peek(&self) -> Option<&ThreadEvent> {
        self.buffer.front()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Get the number of events in the queue.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Clear all events from the queue.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Drain all events, returning them as a vector.
    pub fn drain(&mut self) -> Vec<ThreadEvent> {
        self.buffer.drain(..).collect()
    }
}
