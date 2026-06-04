//! Worker shutdown request and outcome pure types.
//!
//! Describes shutdown requests and outcomes without referencing
//! real transport channels or sync primitives.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerShutdownRequest {
    /// No shutdown requested.
    None,
    /// Request the worker to stop.
    RequestStop,
    /// Close the transport channel.
    CloseTransport,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerShutdownOutcome {
    /// No worker exists to shut down.
    NoWorker,
    /// Stop has been marked (worker will stop when possible).
    StopMarked,
    /// Worker is already stopped.
    AlreadyStopped,
    /// Transport channel has been closed.
    TransportClosed,
    /// Operation is not supported in current state.
    Unsupported,
}

impl OutputThreadWorkerShutdownRequest {
    /// Returns true if this request asks the worker to stop.
    #[allow(dead_code)]
    pub(crate) fn requests_stop(self) -> bool {
        matches!(self, Self::RequestStop)
    }

    /// Returns true if this request closes the transport.
    #[allow(dead_code)]
    pub(crate) fn closes_transport(self) -> bool {
        matches!(self, Self::CloseTransport)
    }
}

impl OutputThreadWorkerShutdownOutcome {
    /// Returns true if the outcome is terminal.
    #[allow(dead_code)]
    pub(crate) fn is_terminal(self) -> bool {
        matches!(self, Self::AlreadyStopped | Self::TransportClosed | Self::Unsupported)
    }

    /// Returns true if the outcome requires a worker to be present.
    #[allow(dead_code)]
    pub(crate) fn requires_worker(self) -> bool {
        matches!(self, Self::StopMarked)
    }
}
