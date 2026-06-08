use super::super::lifecycle::transition::ProductionOutputRouteLifecycleTransitionMatrix;
use super::super::lifecycle::ProductionOutputRouteLifecycleCloseAuthority;
use super::super::lifecycle::ProductionOutputRouteLifecycleCloseDecision as CloseDecision;
use super::super::lifecycle::ProductionOutputRouteLifecycleState as LifecycleState;
use super::super::lifecycle::ProductionOutputRouteLifecycleStateCell;

#[test]
fn close_authority_closes_not_ready_route() {
    let authority = ProductionOutputRouteLifecycleCloseAuthority;
    let mut state_cell =
        ProductionOutputRouteLifecycleStateCell::new(LifecycleState::NotReadyForInput);
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = authority.request_close(&mut state_cell, &matrix);

    assert_eq!(decision, CloseDecision::Closed);
    assert_eq!(state_cell.current_state(), LifecycleState::Closed);
}

#[test]
fn close_authority_closes_accepting_route() {
    let authority = ProductionOutputRouteLifecycleCloseAuthority;
    let mut state_cell =
        ProductionOutputRouteLifecycleStateCell::new(LifecycleState::AcceptingInput);
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = authority.request_close(&mut state_cell, &matrix);

    assert_eq!(decision, CloseDecision::Closed);
    assert_eq!(state_cell.current_state(), LifecycleState::Closed);
}

#[test]
fn close_authority_reports_already_closed_without_update() {
    let authority = ProductionOutputRouteLifecycleCloseAuthority;
    let mut state_cell = ProductionOutputRouteLifecycleStateCell::new(LifecycleState::Closed);
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = authority.request_close(&mut state_cell, &matrix);

    assert_eq!(decision, CloseDecision::AlreadyClosed);
    assert_eq!(state_cell.current_state(), LifecycleState::Closed);
}

#[test]
fn close_authority_decision_is_reasonless_closed_set() {
    // Exhaustive match without wildcard.
    let decision = CloseDecision::Closed;
    match decision {
        CloseDecision::Closed => {}
        CloseDecision::AlreadyClosed => {}
        CloseDecision::Rejected => {}
    }

    // No payload — reasonless.
    let debug = format!("{:?}", decision);
    assert!(!debug.is_empty());

    // Distinguish CloseDecision::Closed from LifecycleState::Closed.
    let _close_decision = CloseDecision::Closed;
    let _lifecycle_state = LifecycleState::Closed;
    // They are different types.
}

#[test]
fn close_authority_request_close_uses_caller_provided_state_and_matrix() {
    let authority = ProductionOutputRouteLifecycleCloseAuthority;
    let mut state_cell =
        ProductionOutputRouteLifecycleStateCell::new(LifecycleState::AcceptingInput);
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // Pass &mut state_cell and &matrix into request_close.
    let _decision = authority.request_close(&mut state_cell, &matrix);
}

#[test]
fn close_authority_request_close_returns_decision_only() {
    let authority = ProductionOutputRouteLifecycleCloseAuthority;
    let mut state_cell =
        ProductionOutputRouteLifecycleStateCell::new(LifecycleState::NotReadyForInput);
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // Assign to explicitly typed variable.
    let decision: CloseDecision = authority.request_close(&mut state_cell, &matrix);

    // Exhaustive match without wildcard.
    match decision {
        CloseDecision::Closed => {}
        CloseDecision::AlreadyClosed => {}
        CloseDecision::Rejected => {}
    }

    // API returns CloseDecision only — no Result/bool/RouteClosed/failure.
}
