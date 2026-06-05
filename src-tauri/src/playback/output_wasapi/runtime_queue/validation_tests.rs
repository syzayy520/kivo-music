use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::entry::OutputThreadRuntimeQueueEntry;
use super::validation::{
    validate_queue_entry_for_runtime, OutputThreadRuntimeQueueValidationError,
};
use super::super::runtime_core::status::OutputThreadRuntimeStatus;
use super::super::output_thread_core::state::OutputThreadState;

#[test]
fn validation_accepts_matching_generation_and_allowed_intent() {
    let gen = OutputThreadRuntimeGeneration::new(1);
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Start, 1);
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Created,
        false,
        false,
        false,
        false,
    );

    assert!(validate_queue_entry_for_runtime(entry, gen, status).is_ok());
}

#[test]
fn validation_rejects_wrong_generation() {
    let gen = OutputThreadRuntimeGeneration::new(1);
    let wrong_gen = OutputThreadRuntimeGeneration::new(2);
    let entry = OutputThreadRuntimeQueueEntry::new(wrong_gen, OutputThreadRuntimeIntent::Start, 1);
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Created,
        false,
        false,
        false,
        false,
    );

    let result = validate_queue_entry_for_runtime(entry, gen, status);
    assert_eq!(
        result.unwrap_err(),
        OutputThreadRuntimeQueueValidationError::WrongGeneration
    );
}

#[test]
fn validation_rejects_intent_not_allowed_for_status() {
    let gen = OutputThreadRuntimeGeneration::new(1);
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Pause, 1);
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Created,
        false,
        false,
        false,
        false,
    );

    let result = validate_queue_entry_for_runtime(entry, gen, status);
    assert_eq!(
        result.unwrap_err(),
        OutputThreadRuntimeQueueValidationError::IntentNotAllowedForStatus
    );
}

#[test]
fn validation_allows_close_for_created_status() {
    let gen = OutputThreadRuntimeGeneration::new(1);
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Close, 1);
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Created,
        false,
        false,
        false,
        false,
    );

    assert!(validate_queue_entry_for_runtime(entry, gen, status).is_ok());
}

#[test]
fn validation_allows_start_for_created_status() {
    let gen = OutputThreadRuntimeGeneration::new(1);
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Start, 1);
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Created,
        false,
        false,
        false,
        false,
    );

    assert!(validate_queue_entry_for_runtime(entry, gen, status).is_ok());
}
