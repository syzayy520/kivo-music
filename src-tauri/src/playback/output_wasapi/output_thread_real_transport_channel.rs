//! Real command transport channel scaffold.
//!
//! This is the only file in this ticket that imports std::sync::mpsc.
//! Creates a typed Sender/Receiver pair for OutputThreadRealTransportCommand.
//!
//! Does not spawn threads, create workers, or reference output/audio APIs.

use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

use super::output_thread_real_transport_command::OutputThreadRealTransportCommand;
use super::output_thread_real_transport_status::{
    OutputThreadRealTransportStatus, OutputThreadRealTransportStatusReport,
};

#[allow(dead_code)]
pub(crate) struct OutputThreadRealTransportChannel {
    sender: Sender<OutputThreadRealTransportCommand>,
    receiver: Receiver<OutputThreadRealTransportCommand>,
    status: OutputThreadRealTransportStatus,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportSendError {
    /// The receiver has been dropped.
    Closed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportRecvResult {
    /// A command was available.
    Command(OutputThreadRealTransportCommand),
    /// No command available (non-blocking).
    Empty,
    /// The sender has been dropped.
    Disconnected,
}

impl OutputThreadRealTransportChannel {
    /// Create a new channel pair. No threads spawned, no workers started.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver,
            status: OutputThreadRealTransportStatus::ChannelCreated,
        }
    }

    /// Return a snapshot status report.
    #[allow(dead_code)]
    pub(crate) fn status_report(&self) -> OutputThreadRealTransportStatusReport {
        OutputThreadRealTransportStatusReport {
            status: self.status,
            has_sender: true,
            has_receiver: true,
            has_worker: false,
            has_thread_handle: false,
            has_device_boundary: false,
        }
    }

    /// Send a command into the channel. Returns Ok(()) on success.
    #[allow(dead_code)]
    pub(crate) fn send_command(
        &self,
        command: OutputThreadRealTransportCommand,
    ) -> Result<(), OutputThreadRealTransportSendError> {
        self.sender
            .send(command)
            .map_err(|_| OutputThreadRealTransportSendError::Closed)
    }

    /// Non-blocking receive. Returns Command, Empty, or Disconnected.
    #[allow(dead_code)]
    pub(crate) fn try_recv_command(&self) -> OutputThreadRealTransportRecvResult {
        match self.receiver.try_recv() {
            Ok(cmd) => OutputThreadRealTransportRecvResult::Command(cmd),
            Err(TryRecvError::Empty) => OutputThreadRealTransportRecvResult::Empty,
            Err(TryRecvError::Disconnected) => OutputThreadRealTransportRecvResult::Disconnected,
        }
    }

    /// Consume the channel and return a closed status report.
    #[allow(dead_code)]
    pub(crate) fn close(self) -> OutputThreadRealTransportStatusReport {
        OutputThreadRealTransportStatusReport::closed()
    }
}

impl Default for OutputThreadRealTransportChannel {
    fn default() -> Self {
        Self::new()
    }
}
