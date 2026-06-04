use super::input::OutputThreadRuntimeQueueBridgeInput;
use super::super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::super::output_thread_state::OutputThreadState;

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
