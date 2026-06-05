//! Transport status model and status report.
//!
//! Tracks channel creation and closure without referencing
//! worker, thread, device, or audio output state.

use super::contract::OutputThreadRealTransportContract;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportStatus {
    /// No transport channel has been created.
    NotCreated,
    /// Sender/receiver pair exists. No worker or thread.
    ChannelCreated,
    /// Transport channel has been closed.
    Closed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRealTransportStatusReport {
    pub status: OutputThreadRealTransportStatus,
    pub has_sender: bool,
    pub has_receiver: bool,
    pub has_worker: bool,
    pub has_thread_handle: bool,
    pub has_device_boundary: bool,
}

impl OutputThreadRealTransportStatusReport {
    /// Build a report from a preflight contract. Always NotCreated.
    #[allow(dead_code)]
    pub(crate) fn from_contract(contract: OutputThreadRealTransportContract) -> Self {
        Self {
            status: OutputThreadRealTransportStatus::NotCreated,
            has_sender: contract.has_sender,
            has_receiver: contract.has_receiver,
            has_worker: false,
            has_thread_handle: false,
            has_device_boundary: false,
        }
    }

    /// Report that channel pair has been created.
    #[allow(dead_code)]
    pub(crate) fn channel_created() -> Self {
        Self {
            status: OutputThreadRealTransportStatus::ChannelCreated,
            has_sender: true,
            has_receiver: true,
            has_worker: false,
            has_thread_handle: false,
            has_device_boundary: false,
        }
    }

    /// Report that transport is closed.
    #[allow(dead_code)]
    pub(crate) fn closed() -> Self {
        Self {
            status: OutputThreadRealTransportStatus::Closed,
            has_sender: false,
            has_receiver: false,
            has_worker: false,
            has_thread_handle: false,
            has_device_boundary: false,
        }
    }

    /// Returns true if status is ChannelCreated.
    #[allow(dead_code)]
    pub(crate) fn is_channel_created(self) -> bool {
        self.status == OutputThreadRealTransportStatus::ChannelCreated
    }

    /// Returns true if no worker or thread handle is present.
    #[allow(dead_code)]
    pub(crate) fn has_no_worker_or_thread(self) -> bool {
        !self.has_worker && !self.has_thread_handle
    }
}
