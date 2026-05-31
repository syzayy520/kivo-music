use super::decoder_runtime_state::{DecoderRuntimePhase, DecoderRuntimeState};

#[test]
fn open_success_transition_is_modeled() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_open();

    assert_eq!(state.phase, DecoderRuntimePhase::Open);
    assert!(state.last_error.is_none());
}

#[test]
fn open_failure_transition_keeps_error_message() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_failed("decoder open failed");

    assert_eq!(state.phase, DecoderRuntimePhase::Failed);
    assert_eq!(state.last_error.as_deref(), Some("decoder open failed"));
}

#[test]
fn close_transition_is_modeled() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_open();
    state.begin_draining();
    state.mark_closed();

    assert_eq!(state.phase, DecoderRuntimePhase::Closed);
}
