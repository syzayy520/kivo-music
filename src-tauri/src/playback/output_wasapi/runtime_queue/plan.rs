use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::entry::OutputThreadRuntimeQueueEntry;
use super::result::{
    OutputThreadRuntimeQueueAcceptResult, OutputThreadRuntimeQueuePlanResult,
    OutputThreadRuntimeQueueRejectReason, OutputThreadRuntimeQueueRejectResult,
};
use super::snapshot::OutputThreadRuntimeQueueSnapshot;

/// Plan whether a queue intent can be accepted.
///
/// Pure function — no real queue, no command sending, no thread interaction.
/// This function only checks queue capacity and closed state.
/// Runtime status validation is handled separately.
#[allow(dead_code)]
pub(crate) fn plan_queue_intent(
    snapshot: OutputThreadRuntimeQueueSnapshot,
    generation: OutputThreadRuntimeGeneration,
    intent: OutputThreadRuntimeIntent,
) -> OutputThreadRuntimeQueuePlanResult {
    if snapshot.state.closed {
        return OutputThreadRuntimeQueuePlanResult::Rejected(OutputThreadRuntimeQueueRejectResult {
            reason: OutputThreadRuntimeQueueRejectReason::Closed,
            state: snapshot.state.with_rejection(),
        });
    }

    if !snapshot.can_accept() {
        return OutputThreadRuntimeQueuePlanResult::Rejected(OutputThreadRuntimeQueueRejectResult {
            reason: OutputThreadRuntimeQueueRejectReason::Full,
            state: snapshot.state.with_rejection(),
        });
    }

    let accepted_state = snapshot.state.with_acceptance();
    let entry = OutputThreadRuntimeQueueEntry::new(generation, intent, accepted_state.last_sequence);

    OutputThreadRuntimeQueuePlanResult::Accepted(OutputThreadRuntimeQueueAcceptResult {
        entry,
        state: accepted_state,
    })
}
