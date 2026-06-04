use super::config::OutputThreadRuntimeQueueConfig;
use super::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::state::OutputThreadRuntimeQueueState;

#[test]
fn empty_snapshot_uses_config() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let snapshot = OutputThreadRuntimeQueueSnapshot::empty(config);

    assert_eq!(snapshot.config, config);
    assert_eq!(snapshot.state, OutputThreadRuntimeQueueState::empty());
}

#[test]
fn snapshot_pending_count_matches_state() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(5, 5, 0, 5, false);
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);

    assert_eq!(snapshot.pending_count(), 5);
}

#[test]
fn snapshot_has_capacity_when_under_limit() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(31, 31, 0, 31, false);
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);

    assert!(snapshot.has_capacity());
}

#[test]
fn snapshot_reports_closed() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::empty().closed();
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);

    assert!(snapshot.is_closed());
}

#[test]
fn snapshot_can_accept_when_open_and_capacity_available() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::empty();
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);

    assert!(snapshot.can_accept());

    let closed_snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state.closed());
    assert!(!closed_snapshot.can_accept());

    let full_state = OutputThreadRuntimeQueueState::new(32, 32, 0, 32, false);
    let full_snapshot = OutputThreadRuntimeQueueSnapshot::new(config, full_state);
    assert!(!full_snapshot.can_accept());
}
