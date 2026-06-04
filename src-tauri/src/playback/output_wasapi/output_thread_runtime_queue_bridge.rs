use super::output_thread_runtime_handle::OutputThreadRuntimeHandle;
use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;
use super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry;
use super::output_thread_runtime_queue_plan::plan_queue_intent;
use super::output_thread_runtime_queue_result::{
    OutputThreadRuntimeQueuePlanResult, OutputThreadRuntimeQueueRejectReason,
};
use super::output_thread_runtime_queue_snapshot::OutputThreadRuntimeQueueSnapshot;
use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;
use super::output_thread_runtime_queue_validation::validate_queue_entry_for_runtime;
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

/// Accepted bridge result.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeAccepted {
    pub entry: OutputThreadRuntimeQueueEntry,
    pub queue_state: OutputThreadRuntimeQueueState,
    pub projection: OutputThreadRuntimeQueueBridgeProjection,
}
/// Rejected bridge result.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueBridgeRejected {
    pub reason: OutputThreadRuntimeQueueRejectReason,
    pub queue_state: OutputThreadRuntimeQueueState,
    pub projection: OutputThreadRuntimeQueueBridgeProjection,
}
/// Result of bridge intent planning.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueBridgeResult {
    Accepted(OutputThreadRuntimeQueueBridgeAccepted),
    Rejected(OutputThreadRuntimeQueueBridgeRejected),
}
#[allow(dead_code)]
impl OutputThreadRuntimeQueueBridgeResult {
    pub(crate) fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted(_))
    }
    pub(crate) fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected(_))
    }
    pub(crate) fn queue_state(self) -> OutputThreadRuntimeQueueState {
        match self {
            Self::Accepted(a) => a.queue_state,
            Self::Rejected(r) => r.queue_state,
        }
    }
    pub(crate) fn projection(self) -> OutputThreadRuntimeQueueBridgeProjection {
        match self {
            Self::Accepted(a) => a.projection,
            Self::Rejected(r) => r.projection,
        }
    }
}

fn projection_with_queue_state(
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

fn reject_bridge(
    input: OutputThreadRuntimeQueueBridgeInput,
    reason: OutputThreadRuntimeQueueRejectReason,
    queue_state: OutputThreadRuntimeQueueState,
) -> OutputThreadRuntimeQueueBridgeResult {
    let projection = projection_with_queue_state(input, queue_state);
    OutputThreadRuntimeQueueBridgeResult::Rejected(OutputThreadRuntimeQueueBridgeRejected {
        reason,
        queue_state,
        projection,
    })
}

/// Plan a runtime queue bridge intent.
#[allow(dead_code)]
pub(crate) fn plan_runtime_queue_bridge_intent(
    input: OutputThreadRuntimeQueueBridgeInput,
    intent: OutputThreadRuntimeIntent,
) -> OutputThreadRuntimeQueueBridgeResult {
    let plan_result = plan_queue_intent(input.queue, input.generation(), intent);
    let accepted = match plan_result {
        OutputThreadRuntimeQueuePlanResult::Rejected(reject) => {
            return reject_bridge(input, reject.reason, reject.state);
        }
        OutputThreadRuntimeQueuePlanResult::Accepted(accept) => accept,
    };
    // Runtime validation
    if validate_queue_entry_for_runtime(accepted.entry, input.generation(), input.status())
        .is_err()
    {
        let rejected_state = accepted.state.with_pending_decrement().with_rejection();
        return reject_bridge(
            input,
            OutputThreadRuntimeQueueRejectReason::InvalidForRuntime,
            rejected_state,
        );
    }
    // Reset policy validation
    if !validate_reset_policy_for_bridge(input, intent) {
        let rejected_state = accepted.state.with_pending_decrement().with_rejection();
        return reject_bridge(
            input,
            OutputThreadRuntimeQueueRejectReason::InvalidForRuntime,
            rejected_state,
        );
    }
    // Accepted
    let projection = projection_with_queue_state(input, accepted.state);
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: accepted.entry,
        queue_state: accepted.state,
        projection,
    })
}

/// Validate reset policy for bridge.
#[allow(dead_code)]
pub(crate) fn validate_reset_policy_for_bridge(
    input: OutputThreadRuntimeQueueBridgeInput,
    intent: OutputThreadRuntimeIntent,
) -> bool {
    if intent != OutputThreadRuntimeIntent::ResetDevice {
        return true;
    }
    if input.state() == OutputThreadState::Stopped
        && !input.queue_config().allow_reset_when_stopped
    {
        return false;
    }
    true
}