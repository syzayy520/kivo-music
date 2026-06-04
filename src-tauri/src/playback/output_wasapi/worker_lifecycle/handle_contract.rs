//! Worker handle contract pure types.
//!
//! Describes ownership and capability of a future worker handle
//! without referencing real thread or sync primitives.

use super::lifecycle::OutputThreadWorkerLifecycleStage;

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerHandleOwnership {
    /// No entity owns the handle.
    NotOwned,
    /// Transport layer owns the handle.
    TransportOwned,
    /// Future worker will own the handle.
    FutureWorkerOwned,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerHandleContract {
    pub lifecycle: OutputThreadWorkerLifecycleStage,
    pub ownership: OutputThreadWorkerHandleOwnership,
    pub has_handle: bool,
    pub can_join: bool,
    pub can_signal_stop: bool,
    pub has_worker_loop: bool,
}

impl OutputThreadWorkerHandleContract {
    /// Create a contract-only instance: no handle, no capabilities.
    #[allow(dead_code)]
    pub(crate) fn contract_only() -> Self {
        Self {
            lifecycle: OutputThreadWorkerLifecycleStage::ContractOnly,
            ownership: OutputThreadWorkerHandleOwnership::NotOwned,
            has_handle: false,
            can_join: false,
            can_signal_stop: false,
            has_worker_loop: false,
        }
    }

    /// Returns true if no handle exists.
    #[allow(dead_code)]
    pub(crate) fn has_no_handle(self) -> bool {
        !self.has_handle
    }

    /// Returns true if no worker loop exists.
    #[allow(dead_code)]
    pub(crate) fn has_no_worker_loop(self) -> bool {
        !self.has_worker_loop
    }

    /// Returns true if join can be performed now.
    /// Currently always false — no real handle to join.
    #[allow(dead_code)]
    pub(crate) fn can_join_now(self) -> bool {
        false
    }

    /// Returns true if stop can be signaled now.
    #[allow(dead_code)]
    pub(crate) fn can_signal_stop_now(self) -> bool {
        self.can_signal_stop
    }
}
