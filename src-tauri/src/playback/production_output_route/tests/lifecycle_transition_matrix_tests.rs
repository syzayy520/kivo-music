use super::super::lifecycle::{
    ProductionOutputRouteLifecycleState as S,
    transition::{
        ProductionOutputRouteLifecycleTransitionDecision as D,
        ProductionOutputRouteLifecycleTransitionMatrix,
    },
};

fn matrix() -> ProductionOutputRouteLifecycleTransitionMatrix {
    ProductionOutputRouteLifecycleTransitionMatrix::new()
}

#[test]
fn transition_matrix_allows_not_ready_to_accepting() {
    let decision = matrix().validate_transition(S::NotReadyForInput, S::AcceptingInput);
    assert_eq!(decision, D::Allowed);
}

#[test]
fn transition_matrix_allows_not_ready_to_closed() {
    let decision = matrix().validate_transition(S::NotReadyForInput, S::Closed);
    assert_eq!(decision, D::Allowed);
}

#[test]
fn transition_matrix_allows_accepting_to_closed() {
    let decision = matrix().validate_transition(S::AcceptingInput, S::Closed);
    assert_eq!(decision, D::Allowed);
}

#[test]
fn transition_matrix_allows_same_state_idempotent_decisions() {
    // Same-state transitions are idempotent validation decisions, not mutation.
    assert_eq!(matrix().validate_transition(S::NotReadyForInput, S::NotReadyForInput), D::Allowed);
    assert_eq!(matrix().validate_transition(S::AcceptingInput, S::AcceptingInput), D::Allowed);
    assert_eq!(matrix().validate_transition(S::Closed, S::Closed), D::Allowed);
}

#[test]
fn transition_matrix_rejects_accepting_to_not_ready() {
    let decision = matrix().validate_transition(S::AcceptingInput, S::NotReadyForInput);
    assert_eq!(decision, D::Rejected);
}

#[test]
fn transition_matrix_rejects_closed_to_accepting() {
    let decision = matrix().validate_transition(S::Closed, S::AcceptingInput);
    assert_eq!(decision, D::Rejected);
}

#[test]
fn transition_matrix_rejects_closed_to_not_ready() {
    let decision = matrix().validate_transition(S::Closed, S::NotReadyForInput);
    assert_eq!(decision, D::Rejected);
}

#[test]
fn transition_decision_is_closed_set_value() {
    // Only Allowed / Rejected exist. Exhaustive match without wildcard.
    let decisions = [D::Allowed, D::Rejected];
    for decision in &decisions {
        match decision {
            D::Allowed => {}
            D::Rejected => {}
        }
    }
    assert_eq!(decisions.len(), 2);
}

#[test]
fn transition_matrix_does_not_mutate_state() {
    // Caller-supplied current/next states remain values.
    // validate_transition returns only decision, not updated state.
    let current = S::AcceptingInput;
    let next = S::Closed;

    let decision = matrix().validate_transition(current, next);
    assert_eq!(decision, D::Allowed);

    // Original values unchanged — no mutation.
    assert_eq!(current, S::AcceptingInput);
    assert_eq!(next, S::Closed);
}

#[test]
fn transition_matrix_api_shape_is_validation_only() {
    // validate_transition takes &self (not &mut self).
    // Returns Decision only — not Result/bool/next state/RouteClosed/failure.
    let m = matrix();
    let decision = m.validate_transition(S::NotReadyForInput, S::AcceptingInput);

    // Decision is a pure value.
    match decision {
        D::Allowed => {}
        D::Rejected => {}
    }
}

#[test]
fn transition_matrix_does_not_duplicate_input_gate() {
    // API has no input parameter — no ProductionOutputRouteFrameInput usage.
    // No RouteClosed construction inside matrix.
    let decision = matrix().validate_transition(S::AcceptingInput, S::Closed);
    assert_eq!(decision, D::Allowed);
}
