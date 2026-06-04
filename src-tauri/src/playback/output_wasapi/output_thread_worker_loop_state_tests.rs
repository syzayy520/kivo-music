use super::output_thread_worker_loop_state::OutputThreadWorkerLoopState;

#[test]
fn transport_closed_is_terminal() {
    assert!(OutputThreadWorkerLoopState::TransportClosed.is_terminal());
}

#[test]
fn stop_requested_is_terminal() {
    assert!(OutputThreadWorkerLoopState::StopRequested.is_terminal());
}

#[test]
fn not_started_is_not_terminal() {
    assert!(!OutputThreadWorkerLoopState::NotStarted.is_terminal());
}

#[test]
fn not_started_can_poll() {
    assert!(OutputThreadWorkerLoopState::NotStarted.can_poll());
}

#[test]
fn transport_closed_cannot_poll() {
    assert!(!OutputThreadWorkerLoopState::TransportClosed.can_poll());
}

#[test]
fn stop_requested_cannot_poll() {
    assert!(!OutputThreadWorkerLoopState::StopRequested.can_poll());
}

#[test]
fn state_names_do_not_imply_ready_or_running() {
    let all = [
        OutputThreadWorkerLoopState::NotStarted,
        OutputThreadWorkerLoopState::Polling,
        OutputThreadWorkerLoopState::NoCommand,
        OutputThreadWorkerLoopState::CommandHandled,
        OutputThreadWorkerLoopState::StopRequested,
        OutputThreadWorkerLoopState::TransportClosed,
    ];
    for state in all {
        let name = format!("{:?}", state);
        assert!(!name.to_lowercase().contains("ready"), "state {:?}", state);
        assert!(
            !name.to_lowercase().contains("running"),
            "state {:?}",
            state
        );
    }
}

#[test]
fn has_worker_thread_is_false_for_all_states() {
    let all = [
        OutputThreadWorkerLoopState::NotStarted,
        OutputThreadWorkerLoopState::Polling,
        OutputThreadWorkerLoopState::NoCommand,
        OutputThreadWorkerLoopState::CommandHandled,
        OutputThreadWorkerLoopState::StopRequested,
        OutputThreadWorkerLoopState::TransportClosed,
    ];
    for state in all {
        assert!(!state.has_worker_thread(), "state {:?}", state);
    }
}

#[test]
fn all_variants_debug_and_copy() {
    let a = OutputThreadWorkerLoopState::NotStarted;
    let b = a;
    assert_eq!(a, b);
    let _ = format!("{:?}", a);
}
