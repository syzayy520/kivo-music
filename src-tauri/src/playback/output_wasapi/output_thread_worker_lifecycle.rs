//! Worker lifecycle stage and reason pure types.
//!
//! Describes the lifecycle phases of a future worker without
//! referencing any real thread, sync, or audio primitives.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLifecycleStage {
    /// No worker structure has been created.
    NotCreated,
    /// Contract descriptor exists, but no real worker.
    ContractOnly,
    /// Handle contract exists, but worker not started.
    HandleNotStarted,
    /// A stop has been requested but worker not yet stopped.
    StopRequested,
    /// Worker has been stopped.
    Stopped,
    /// Worker encountered a failure.
    Failed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLifecycleReason {
    /// Worker was never started.
    NotStarted,
    /// Stop was explicitly requested.
    StopRequested,
    /// Transport channel was closed.
    ClosedTransport,
    /// Preflight check failed.
    FailedPreflight,
    /// Worker is unavailable for operation.
    WorkerUnavailable,
}

impl OutputThreadWorkerLifecycleStage {
    /// Returns true if the stage is terminal (no further transitions).
    #[allow(dead_code)]
    pub(crate) fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped | Self::Failed)
    }

    /// Returns true if a stop can be requested from this stage.
    #[allow(dead_code)]
    pub(crate) fn can_request_stop(self) -> bool {
        !self.is_terminal() && !matches!(self, Self::NotCreated)
    }

    /// Returns true if a live worker exists at this stage.
    /// Currently always false — no real worker exists.
    #[allow(dead_code)]
    pub(crate) fn has_live_worker(self) -> bool {
        false
    }
}
