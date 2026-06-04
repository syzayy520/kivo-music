//! Preflight contract for future real command transport.
//!
//! This module describes the lifecycle stages, ownership roles,
//! and missing prerequisites for a real WASAPI transport.
//!
//! Currently all values default to "not started" — no transport
//! exists, no sender/receiver, no worker, no thread, no device.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportStage {
    /// No transport structure has been created yet.
    NotCreated,
    /// Contract descriptor exists, but no real transport.
    ContractOnly,
    /// Transport structure exists but is not running.
    TransportNotStarted,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportOwnerRole {
    /// Sink owns the transport lifecycle.
    SinkOwned,
    /// Worker thread owns the transport lifecycle.
    WorkerOwned,
    /// Report-only: no ownership, observation only.
    ReportOnly,
}

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportUnsupportedReason {
    /// No mpsc::Sender for commands.
    NoSender,
    /// No mpsc::Receiver for commands.
    NoReceiver,
    /// No worker function or loop.
    NoWorker,
    /// No std::thread::JoinHandle.
    NoThread,
    /// No WASAPI device boundary established.
    NoDevice,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRealTransportContract {
    pub stage: OutputThreadRealTransportStage,
    pub owner_role: OutputThreadRealTransportOwnerRole,
    pub has_sender: bool,
    pub has_receiver: bool,
    pub has_worker: bool,
    pub has_thread_handle: bool,
    pub has_device_boundary: bool,
}

impl OutputThreadRealTransportContract {
    /// Create a contract-only instance: no transport primitives exist.
    #[allow(dead_code)]
    pub(crate) fn contract_only() -> Self {
        Self {
            stage: OutputThreadRealTransportStage::ContractOnly,
            owner_role: OutputThreadRealTransportOwnerRole::ReportOnly,
            has_sender: false,
            has_receiver: false,
            has_worker: false,
            has_thread_handle: false,
            has_device_boundary: false,
        }
    }

    /// Returns true if stage is ContractOnly.
    #[allow(dead_code)]
    pub(crate) fn is_contract_only(self) -> bool {
        self.stage == OutputThreadRealTransportStage::ContractOnly
    }

    /// Returns true if a real transport has been started.
    /// Currently always false — no real transport exists.
    #[allow(dead_code)]
    pub(crate) fn is_transport_started(self) -> bool {
        false
    }

    /// Returns true if no runtime primitives (sender, receiver,
    /// worker, thread handle, device boundary) are present.
    #[allow(dead_code)]
    pub(crate) fn has_no_runtime_primitives(self) -> bool {
        !self.has_sender
            && !self.has_receiver
            && !self.has_worker
            && !self.has_thread_handle
            && !self.has_device_boundary
    }

    /// Returns fixed array of all unsupported reasons.
    /// Always returns all 5 — no transport can start.
    #[allow(dead_code)]
    pub(crate) fn unsupported_reasons(self) -> [OutputThreadRealTransportUnsupportedReason; 5] {
        [
            OutputThreadRealTransportUnsupportedReason::NoSender,
            OutputThreadRealTransportUnsupportedReason::NoReceiver,
            OutputThreadRealTransportUnsupportedReason::NoWorker,
            OutputThreadRealTransportUnsupportedReason::NoThread,
            OutputThreadRealTransportUnsupportedReason::NoDevice,
        ]
    }
}
