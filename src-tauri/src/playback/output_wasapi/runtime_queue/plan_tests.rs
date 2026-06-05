use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::config::OutputThreadRuntimeQueueConfig;
use super::plan::plan_queue_intent;
use super::result::OutputThreadRuntimeQueuePlanResult;
use super::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::state::OutputThreadRuntimeQueueState;

#[test]
fn plan_accepts_when_open_and_has_capacity() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let snapshot = OutputThreadRuntimeQueueSnapshot::empty(config);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    assert!(result.is_accepted());
}

#[test]
fn plan_rejects_when_closed() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::empty().closed();
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    assert!(result.is_rejected());
}

#[test]
fn plan_rejects_when_full() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(32, 32, 0, 32, false);
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    assert!(result.is_rejected());
}

#[test]
fn accepted_entry_uses_next_sequence() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(0, 5, 0, 5, false);
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    if let OutputThreadRuntimeQueuePlanResult::Accepted(accepted) = result {
        assert_eq!(accepted.entry.sequence, 6);
        assert_eq!(accepted.state.last_sequence, 6);
    } else {
        panic!("expected acceptance");
    }
}

#[test]
fn accepted_entry_uses_generation() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let snapshot = OutputThreadRuntimeQueueSnapshot::empty(config);
    let gen = OutputThreadRuntimeGeneration::new(7);

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    if let OutputThreadRuntimeQueuePlanResult::Accepted(accepted) = result {
        assert_eq!(accepted.entry.generation, gen);
    } else {
        panic!("expected acceptance");
    }
}

#[test]
fn accepted_entry_preserves_intent() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let snapshot = OutputThreadRuntimeQueueSnapshot::empty(config);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Flush);
    if let OutputThreadRuntimeQueuePlanResult::Accepted(accepted) = result {
        assert_eq!(accepted.entry.intent, OutputThreadRuntimeIntent::Flush);
    } else {
        panic!("expected acceptance");
    }
}

#[test]
fn rejected_plan_increments_rejected_count() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::empty().closed();
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let gen = OutputThreadRuntimeGeneration::default();

    let result = plan_queue_intent(snapshot, gen, OutputThreadRuntimeIntent::Start);
    assert_eq!(result.state().rejected_count, 1);
}
