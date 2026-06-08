//! Command queue type.
//!
//! FIFO queue for ThreadCommand. Uses VecDeque internally.
//! No threads, no channels, no side effects.

use std::collections::VecDeque;

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;

/// FIFO command queue for the output thread.
#[derive(Debug, Clone, Default)]
pub struct CommandQueue {
    /// Internal buffer.
    buffer: VecDeque<ThreadCommand>,
}

impl CommandQueue {
    /// Create a new empty command queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a command to the back of the queue.
    pub fn push(&mut self, command: ThreadCommand) {
        self.buffer.push_back(command);
    }

    /// Pop a command from the front of the queue.
    pub fn pop(&mut self) -> Option<ThreadCommand> {
        self.buffer.pop_front()
    }

    /// Peek at the front command without removing it.
    pub fn peek(&self) -> Option<&ThreadCommand> {
        self.buffer.front()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Get the number of commands in the queue.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Clear all commands from the queue.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Drain all commands, returning them as a vector.
    pub fn drain(&mut self) -> Vec<ThreadCommand> {
        self.buffer.drain(..).collect()
    }
}
