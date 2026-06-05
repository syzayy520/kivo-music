use super::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::status::OutputThreadRuntimeStatus;
use super::super::output_thread_state::OutputThreadState;

/// Descriptor for the output thread runtime handle.
///
/// This is a pure descriptor — not a real thread handle.
/// No thread primitives.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeHandle {
    /// Unique identifier for this runtime instance.
    pub id: OutputThreadRuntimeId,
    /// Generation counter for successive lifetimes.
    pub generation: OutputThreadRuntimeGeneration,
    /// Current runtime status.
    pub status: OutputThreadRuntimeStatus,
}

#[allow(dead_code)]
impl OutputThreadRuntimeHandle {
    /// Create a new runtime handle descriptor.
    pub(crate) fn new(
        id: OutputThreadRuntimeId,
        generation: OutputThreadRuntimeGeneration,
        status: OutputThreadRuntimeStatus,
    ) -> Self {
        Self {
            id,
            generation,
            status,
        }
    }

    /// Create an inactive handle descriptor.
    pub(crate) fn inactive() -> Self {
        Self {
            id: OutputThreadRuntimeId::default(),
            generation: OutputThreadRuntimeGeneration::default(),
            status: OutputThreadRuntimeStatus::inactive(),
        }
    }

    /// Whether the handle is attached to a runtime.
    pub(crate) fn is_attached(self) -> bool {
        self.status.has_handle
    }

    /// Whether the runtime is currently running.
    pub(crate) fn is_running(self) -> bool {
        self.status.is_running()
    }

    /// Whether the runtime can accept audio frames right now.
    pub(crate) fn can_accept_frames(self) -> bool {
        self.status.can_accept_frames()
    }

    /// Replace the runtime status.
    pub(crate) fn with_status(mut self, status: OutputThreadRuntimeStatus) -> Self {
        self.status = status;
        self
    }

    /// Replace the lifecycle state within the status.
    pub(crate) fn with_state(mut self, state: OutputThreadState) -> Self {
        self.status.state = state;
        self
    }
}
