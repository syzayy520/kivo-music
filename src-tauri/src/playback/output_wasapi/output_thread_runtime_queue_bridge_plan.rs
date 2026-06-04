use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_queue_bridge_input::OutputThreadRuntimeQueueBridgeInput;
use super::output_thread_runtime_queue_bridge_projection::projection_with_queue_state;
use super::output_thread_runtime_queue_bridge_result::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeRejected,
    OutputThreadRuntimeQueueBridgeResult,
};
use super::output_thread_runtime_queue_bridge_policy::validate_reset_policy_for_bridge;
use super::runtime_queue::plan::plan_queue_intent;
use super::runtime_queue::result::{
    OutputThreadRuntimeQueuePlanResult, OutputThreadRuntimeQueueRejectReason,
};
use super::runtime_queue::validation::validate_queue_entry_for_runtime;

fn reject_bridge(
    input: OutputThreadRuntimeQueueBridgeInput,
    reason: OutputThreadRuntimeQueueRejectReason,
    queue_state: super::runtime_queue::state::OutputThreadRuntimeQueueState,
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
    if !validate_reset_policy_for_bridge(input, intent) {
        let rejected_state = accepted.state.with_pending_decrement().with_rejection();
        return reject_bridge(
            input,
            OutputThreadRuntimeQueueRejectReason::InvalidForRuntime,
            rejected_state,
        );
    }
    let projection = projection_with_queue_state(input, accepted.state);
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: accepted.entry,
        queue_state: accepted.state,
        projection,
    })
}
