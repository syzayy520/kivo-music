//! Command sender boundary.
//!
//! Wraps std::sync::mpsc::Sender<ThreadCommand> for type-safe
//! command dispatch to the output thread.

use std::sync::mpsc;

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;

/// Type-safe command sender wrapping mpsc::Sender.
#[derive(Debug, Clone)]
pub struct CommandSender {
    inner: mpsc::Sender<ThreadCommand>,
}

impl CommandSender {
    /// Create a new CommandSender wrapping the given mpsc sender.
    pub fn new(sender: mpsc::Sender<ThreadCommand>) -> Self {
        Self { inner: sender }
    }

    /// Send a command to the output thread.
    ///
    /// Returns Ok(()) on success, Err if the receiver has been dropped.
    pub fn send(&self, command: ThreadCommand) -> Result<(), mpsc::SendError<ThreadCommand>> {
        self.inner.send(command)
    }
}

/// Create a bounded command channel pair.
pub fn command_channel() -> (CommandSender, CommandReceiver) {
    let (tx, rx) = mpsc::channel();
    (CommandSender::new(tx), CommandReceiver::new(rx))
}

// Import CommandReceiver for the channel factory function.
use super::command_receiver::CommandReceiver;
