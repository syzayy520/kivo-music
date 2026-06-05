use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::entry::OutputThreadRuntimeQueueEntry;

#[test]
fn entry_new_preserves_fields() {
    let gen = OutputThreadRuntimeGeneration::new(3);
    let intent = OutputThreadRuntimeIntent::Start;
    let entry = OutputThreadRuntimeQueueEntry::new(gen, intent, 42);

    assert_eq!(entry.generation, gen);
    assert_eq!(entry.intent, intent);
    assert_eq!(entry.sequence, 42);
}

#[test]
fn entry_matches_generation() {
    let gen = OutputThreadRuntimeGeneration::new(5);
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Stop, 1);

    assert!(entry.is_for_generation(gen));
    assert!(!entry.is_for_generation(OutputThreadRuntimeGeneration::new(6)));
}

#[test]
fn entry_detects_shutdown_request() {
    let gen = OutputThreadRuntimeGeneration::default();

    let stop = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Stop, 1);
    assert!(stop.is_shutdown_request());

    let close = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Close, 2);
    assert!(close.is_shutdown_request());

    let start = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Start, 3);
    assert!(!start.is_shutdown_request());
}

#[test]
fn entry_detects_buffer_clearing_intent() {
    let gen = OutputThreadRuntimeGeneration::default();

    let flush = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Flush, 1);
    assert!(flush.clears_buffer());

    let reset = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::ResetDevice, 2);
    assert!(reset.clears_buffer());

    let start = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Start, 3);
    assert!(!start.clears_buffer());
}

#[test]
fn entry_next_sequence_increments() {
    let gen = OutputThreadRuntimeGeneration::default();
    let entry = OutputThreadRuntimeQueueEntry::new(gen, OutputThreadRuntimeIntent::Start, 10);
    assert_eq!(entry.next_sequence(), 11);
}
