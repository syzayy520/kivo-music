use super::input::OutputThreadRuntimeQueueBridgeInput;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::super::output_thread_core::state::OutputThreadState;

/// Projection of runtime queue bridge state.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeProjection {
    pub runtime_state: OutputThreadState,
    pub pending_count: u16,
    pub accepted_count: u64,
    pub rejected_count: u64,
    pub last_sequence: u64,
    pub queue_closed: bool,
    pub queue_has_capacity: bool,
    pub runtime_can_accept_frames: bool,
    pub bridge_can_accept_intents: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueBridgeProjection {
    pub(crate) fn can_accept_intents(self) -> bool {
        self.bridge_can_accept_intents
    }
    pub(crate) fn has_pending_intents(self) -> bool {
        self.pending_count > 0
    }
    pub(crate) fn is_closed(self) -> bool {
        self.queue_closed
    }
}

pub(crate) fn projection_with_queue_state(
    input: OutputThreadRuntimeQueueBridgeInput,
    queue_state: OutputThreadRuntimeQueueState,
) -> OutputThreadRuntimeQueueBridgeProjection {
    let queue_has_capacity =
        queue_state.pending_count < input.queue.config.max_pending_commands;
    let bridge_can_accept_intents = !queue_state.closed && queue_has_capacity;
    OutputThreadRuntimeQueueBridgeProjection {
        runtime_state: input.handle.status.state,
        pending_count: queue_state.pending_count,
        accepted_count: queue_state.accepted_count,
        rejected_count: queue_state.rejected_count,
        last_sequence: queue_state.last_sequence,
        queue_closed: queue_state.closed,
        queue_has_capacity,
        runtime_can_accept_frames: input.handle.status.can_accept_frames(),
        bridge_can_accept_intents,
    }
}
