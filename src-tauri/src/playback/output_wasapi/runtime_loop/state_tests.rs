use super::state::OutputThreadRuntimeLoopState;

#[test]
fn idle_can_accept_start() {
    assert!(OutputThreadRuntimeLoopState::Idle.can_accept_start());
}

#[test]
fn stopped_can_accept_start() {
    assert!(OutputThreadRuntimeLoopState::Stopped.can_accept_start());
}

#[test]
fn active_can_accept_stop() {
    assert!(OutputThreadRuntimeLoopState::Active.can_accept_stop());
}

#[test]
fn exited_is_terminal() {
    assert!(OutputThreadRuntimeLoopState::Exited.is_terminal());
}

#[test]
fn stopping_is_not_startable() {
    assert!(!OutputThreadRuntimeLoopState::Stopping.can_accept_start());
}

#[test]
fn active_is_active() {
    assert!(OutputThreadRuntimeLoopState::Active.is_active());
}

#[test]
fn idle_is_not_active() {
    assert!(!OutputThreadRuntimeLoopState::Idle.is_active());
}

#[test]
fn idle_is_not_terminal() {
    assert!(!OutputThreadRuntimeLoopState::Idle.is_terminal());
}

#[test]
fn active_cannot_accept_start() {
    assert!(!OutputThreadRuntimeLoopState::Active.can_accept_start());
}

#[test]
fn idle_cannot_accept_stop() {
    assert!(!OutputThreadRuntimeLoopState::Idle.can_accept_stop());
}