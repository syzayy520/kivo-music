use super::decoder_runtime_state::{DecoderRuntimePhase, DecoderRuntimeState};

fn assert_state(
    state: &DecoderRuntimeState,
    expected_phase: DecoderRuntimePhase,
    expected_error: Option<&str>,
) {
    assert_eq!(state.phase, expected_phase);
    assert_eq!(state.last_error.as_deref(), expected_error);
}

#[test]
fn idle_state_starts_without_error() {
    let state = DecoderRuntimeState::idle();

    assert_state(&state, DecoderRuntimePhase::Idle, None);
}

#[test]
fn open_success_transition_is_modeled() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_open();

    assert_state(&state, DecoderRuntimePhase::Open, None);
}

#[test]
fn open_failure_transition_keeps_error_message() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_failed("decoder open failed");

    assert_state(
        &state,
        DecoderRuntimePhase::Failed,
        Some("decoder open failed"),
    );
}

#[test]
fn close_transition_is_modeled() {
    let mut state = DecoderRuntimeState::idle();

    state.begin_opening();
    state.mark_open();
    state.begin_draining();
    state.mark_closed();

    assert_state(&state, DecoderRuntimePhase::Closed, None);
}

#[test]
fn begin_opening_after_failed_clears_error() {
    let mut state = DecoderRuntimeState::idle();

    state.mark_failed("decoder open failed");
    state.begin_opening();

    assert_state(&state, DecoderRuntimePhase::Opening, None);
}

#[test]
fn mark_open_after_failed_clears_error() {
    let mut state = DecoderRuntimeState::idle();

    state.mark_failed("decoder open failed");
    state.mark_open();

    assert_state(&state, DecoderRuntimePhase::Open, None);
}

#[test]
fn draining_and_closed_preserve_error_context() {
    let mut state = DecoderRuntimeState::idle();

    state.mark_failed("decode failed");
    state.begin_draining();
    assert_state(&state, DecoderRuntimePhase::Draining, Some("decode failed"));

    state.mark_closed();
    assert_state(&state, DecoderRuntimePhase::Closed, Some("decode failed"));
}

#[test]
fn mark_failed_replaces_previous_error_message() {
    let mut state = DecoderRuntimeState::idle();

    state.mark_failed("first failure");
    state.mark_failed("second failure");

    assert_state(&state, DecoderRuntimePhase::Failed, Some("second failure"));
}
