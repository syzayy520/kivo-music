use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;

#[test]
fn empty_state_has_no_pending() {
    let state = OutputThreadRuntimeQueueState::empty();
    assert!(state.is_empty());
    assert_eq!(state.pending_count, 0);
    assert_eq!(state.accepted_count, 0);
    assert_eq!(state.rejected_count, 0);
    assert_eq!(state.last_sequence, 0);
    assert!(!state.closed);
}

#[test]
fn state_can_accept_below_capacity() {
    let state = OutputThreadRuntimeQueueState::empty();
    assert!(state.can_accept(32));
}

#[test]
fn state_cannot_accept_when_full() {
    let state = OutputThreadRuntimeQueueState::new(32, 32, 0, 32, false);
    assert!(!state.can_accept(32));
}

#[test]
fn state_cannot_accept_when_closed() {
    let state = OutputThreadRuntimeQueueState::empty().closed();
    assert!(!state.can_accept(32));
}

#[test]
fn acceptance_increments_pending_and_counts() {
    let state = OutputThreadRuntimeQueueState::empty();
    let accepted = state.with_acceptance();

    assert_eq!(accepted.pending_count, 1);
    assert_eq!(accepted.accepted_count, 1);
    assert_eq!(accepted.last_sequence, 1);
    assert_eq!(accepted.rejected_count, 0);
}

#[test]
fn rejection_increments_rejected_count() {
    let state = OutputThreadRuntimeQueueState::empty();
    let rejected = state.with_rejection();

    assert_eq!(rejected.pending_count, 0);
    assert_eq!(rejected.accepted_count, 0);
    assert_eq!(rejected.rejected_count, 1);
}

#[test]
fn pending_decrement_saturates_at_zero() {
    let state = OutputThreadRuntimeQueueState::empty();
    let decremented = state.with_pending_decrement();
    assert_eq!(decremented.pending_count, 0);

    let state = OutputThreadRuntimeQueueState::new(5, 5, 0, 5, false);
    let decremented = state.with_pending_decrement();
    assert_eq!(decremented.pending_count, 4);
}

#[test]
fn closed_marks_state_closed() {
    let state = OutputThreadRuntimeQueueState::empty();
    assert!(!state.is_closed());

    let closed = state.closed();
    assert!(closed.is_closed());
    assert!(closed.closed);
}
