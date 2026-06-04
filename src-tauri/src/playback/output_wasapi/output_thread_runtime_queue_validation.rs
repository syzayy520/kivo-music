use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry;
use super::output_thread_runtime_status::OutputThreadRuntimeStatus;

/// Validation error for a queue entry against the current runtime.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueValidationError {
    /// Entry targets a different generation than expected.
    WrongGeneration,
    /// Entry intent is not allowed for the current runtime status.
    IntentNotAllowedForStatus,
}

/// Result of validating a queue entry.
#[allow(dead_code)]
pub(crate) type OutputThreadRuntimeQueueValidationResult =
    Result<(), OutputThreadRuntimeQueueValidationError>;

/// Validate that a queue entry is compatible with the current runtime.
///
/// Pure function — no command sending, no state mutation.
#[allow(dead_code)]
pub(crate) fn validate_queue_entry_for_runtime(
    entry: OutputThreadRuntimeQueueEntry,
    expected_generation: OutputThreadRuntimeGeneration,
    status: OutputThreadRuntimeStatus,
) -> OutputThreadRuntimeQueueValidationResult {
    if entry.generation != expected_generation {
        return Err(OutputThreadRuntimeQueueValidationError::WrongGeneration);
    }

    if !entry.intent.can_apply_to(status) {
        return Err(OutputThreadRuntimeQueueValidationError::IntentNotAllowedForStatus);
    }

    Ok(())
}
