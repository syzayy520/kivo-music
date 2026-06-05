//! Real transport handle for spawned output thread.
//!
//! Holds the sender half of the command channel and the thread join handle.
//! Does not reference WASAPI, COM, audio, or async primitives.

use std::sync::mpsc::Sender;
use std::thread;

use super::command::OutputThreadRealTransportCommand;
use super::thread_error::RealOutputThreadSkeletonError;
use super::thread_report::RealOutputThreadReport;

/// Handle for a spawned real output thread.
///
/// Owns the sender side of the command channel and the thread join handle.
/// Does not store audio state, buffer references, or device handles.
#[allow(dead_code)]
pub(crate) struct OutputThreadRealTransportHandle {
    sender: Sender<OutputThreadRealTransportCommand>,
    join_handle:
        Option<thread::JoinHandle<Result<RealOutputThreadReport, RealOutputThreadSkeletonError>>>,
    thread_id: thread::ThreadId,
}

impl OutputThreadRealTransportHandle {
    /// Create a new handle from a sender, join handle, and thread ID.
    #[allow(dead_code)]
    pub(crate) fn new(
        sender: Sender<OutputThreadRealTransportCommand>,
        join_handle: thread::JoinHandle<
            Result<RealOutputThreadReport, RealOutputThreadSkeletonError>,
        >,
    ) -> Self {
        let thread_id = join_handle.thread().id();
        Self {
            sender,
            join_handle: Some(join_handle),
            thread_id,
        }
    }

    /// Get the thread ID.
    #[allow(dead_code)]
    pub(crate) fn thread_id(&self) -> thread::ThreadId {
        self.thread_id
    }

    /// Check if the join handle is still held (thread not yet joined).
    #[allow(dead_code)]
    pub(crate) fn has_join_handle(&self) -> bool {
        self.join_handle.is_some()
    }

    /// Send a command to the thread.
    #[allow(dead_code)]
    pub(crate) fn send_command(
        &self,
        command: OutputThreadRealTransportCommand,
    ) -> Result<(), super::channel::OutputThreadRealTransportSendError> {
        self.sender
            .send(command)
            .map_err(|_| super::channel::OutputThreadRealTransportSendError::Closed)
    }

    /// Take the join handle, consuming it.
    #[allow(dead_code)]
    pub(crate) fn take_join_handle(
        &mut self,
    ) -> Option<thread::JoinHandle<Result<RealOutputThreadReport, RealOutputThreadSkeletonError>>>
    {
        self.join_handle.take()
    }
}
