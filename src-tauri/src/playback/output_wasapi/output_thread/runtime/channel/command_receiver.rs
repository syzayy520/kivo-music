//! Command receiver boundary.
//!
//! Wraps std::sync::mpsc::Receiver<ThreadCommand> for type-safe
//! command reception inside the output thread.

use std::sync::mpsc;

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;

/// Type-safe command receiver wrapping mpsc::Receiver.
#[derive(Debug)]
pub struct CommandReceiver {
    inner: mpsc::Receiver<ThreadCommand>,
}

impl CommandReceiver {
    /// Create a new CommandReceiver wrapping the given mpsc receiver.
    pub fn new(receiver: mpsc::Receiver<ThreadCommand>) -> Self {
        Self { inner: receiver }
    }

    /// Try to receive a command without blocking.
    ///
    /// Returns Ok(command) if available, Err(TryRecvError) otherwise.
    pub fn try_recv(&self) -> Result<ThreadCommand, mpsc::TryRecvError> {
        self.inner.try_recv()
    }

    /// Receive a command, blocking until one is available.
    ///
    /// Returns Ok(command) if available, Err if the sender has been dropped.
    pub fn recv(&self) -> Result<ThreadCommand, mpsc::RecvError> {
        self.inner.recv()
    }
}
