use super::output_thread_runtime_id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::output_thread_runtime_snapshot::OutputThreadRuntimeSnapshot;
use super::output_thread_runtime_status::OutputThreadRuntimeStatus;
use super::output_thread_state::{OutputThreadState, OutputThreadStats};

#[test]
fn empty_snapshot_is_inactive() {
    let snapshot = OutputThreadRuntimeSnapshot::empty();
    assert!(!snapshot.is_active());
    assert!(!snapshot.can_accept_frames());
}

#[test]
fn new_snapshot_preserves_fields() {
    let id = OutputThreadRuntimeId::new(42);
    let gen = OutputThreadRuntimeGeneration::new(3);
    let status = OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, true, true);
    let mut stats = OutputThreadStats::default();
    stats.consumed_frames = 100;

    let snapshot = OutputThreadRuntimeSnapshot::new(id, gen, status, stats);
    assert_eq!(snapshot.id, id);
    assert_eq!(snapshot.generation, gen);
    assert_eq!(snapshot.status, status);
    assert_eq!(snapshot.stats.consumed_frames, 100);
}

#[test]
fn active_snapshot_reports_can_accept_frames() {
    let status =
        OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, false, true);
    let snapshot = OutputThreadRuntimeSnapshot::new(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        status,
        OutputThreadStats::default(),
    );
    assert!(snapshot.is_active());
    assert!(snapshot.can_accept_frames());
}

#[test]
fn generation_accessor_returns_generation() {
    let gen = OutputThreadRuntimeGeneration::new(5);
    let snapshot = OutputThreadRuntimeSnapshot::new(
        OutputThreadRuntimeId::new(1),
        gen,
        OutputThreadRuntimeStatus::inactive(),
        OutputThreadStats::default(),
    );
    assert_eq!(snapshot.generation(), gen);
}
