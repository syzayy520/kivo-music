use super::output_thread_runtime_handle::OutputThreadRuntimeHandle;
use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;
use super::output_thread_runtime_queue_snapshot::OutputThreadRuntimeQueueSnapshot;
use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;
use super::output_thread_runtime_status::OutputThreadRuntimeStatus;
use super::output_thread_state::OutputThreadState;

/// Bridge input combining handle and queue snapshot.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeInput {
    pub handle: OutputThreadRuntimeHandle,
    pub queue: OutputThreadRuntimeQueueSnapshot,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueBridgeInput {
    pub(crate) fn new(
        handle: OutputThreadRuntimeHandle,
        queue: OutputThreadRuntimeQueueSnapshot,
    ) -> Self {
        Self { handle, queue }
    }
    pub(crate) fn generation(self) -> OutputThreadRuntimeGeneration {
        self.handle.generation
    }
    pub(crate) fn status(self) -> OutputThreadRuntimeStatus {
        self.handle.status
    }
    pub(crate) fn state(self) -> OutputThreadState {
        self.handle.status.state
    }
    pub(crate) fn queue_state(self) -> OutputThreadRuntimeQueueState {
        self.queue.state
    }
    pub(crate) fn queue_config(self) -> OutputThreadRuntimeQueueConfig {
        self.queue.config
    }
}
