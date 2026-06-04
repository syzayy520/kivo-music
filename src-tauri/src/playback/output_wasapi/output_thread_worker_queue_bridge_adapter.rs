//! Worker queue bridge adapter — maps observed intent to queue bridge plan.
//!
//! Calls plan_runtime_queue_bridge_intent with the observed intent
//! and classifies the result. Does not pass results to runtime loop,
//! poll transport channels, or reference audio/output primitives.

use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_queue_bridge::plan_runtime_queue_bridge_intent;
use super::output_thread_runtime_queue_bridge_input::OutputThreadRuntimeQueueBridgeInput;
use super::output_thread_runtime_queue_bridge_result::OutputThreadRuntimeQueueBridgeResult;
use super::output_thread_worker_queue_bridge_decision::OutputThreadWorkerQueueBridgeDecision;

/// Result of adapting an observed intent through the queue bridge plan.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerQueueBridgeAdapterResult {
    pub queue_bridge_result: OutputThreadRuntimeQueueBridgeResult,
    pub decision: OutputThreadWorkerQueueBridgeDecision,
    pub queue_bridge_used: bool,
    pub passes_to_runtime_loop: bool,
    pub has_output_behavior: bool,
}

/// Adapt an observed runtime intent through the queue bridge plan.
///
/// Does not poll transport channels or pass results to runtime loop.
/// Always returns queue_bridge_used=true, passes_to_runtime_loop=false,
/// has_output_behavior=false.
#[allow(dead_code)]
pub(crate) fn adapt_worker_intent_to_queue_bridge(
    input: OutputThreadRuntimeQueueBridgeInput,
    observed_runtime_intent: OutputThreadRuntimeIntent,
) -> OutputThreadWorkerQueueBridgeAdapterResult {
    let result = plan_runtime_queue_bridge_intent(input, observed_runtime_intent);
    let decision =
        OutputThreadWorkerQueueBridgeDecision::from_bridge_result(observed_runtime_intent, result);

    OutputThreadWorkerQueueBridgeAdapterResult {
        queue_bridge_result: result,
        decision,
        queue_bridge_used: true,
        passes_to_runtime_loop: false,
        has_output_behavior: false,
    }
}
