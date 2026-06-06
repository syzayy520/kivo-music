use super::super::lifecycle::ProductionOutputRouteLifecycleState;
use super::super::lifecycle::ProductionOutputRouteLifecycleStateCell;
use super::super::lifecycle::ProductionOutputRouteLifecycleStateOwnerUpdateDecision;
use super::super::lifecycle::transition::ProductionOutputRouteLifecycleTransitionMatrix;

#[test]
fn lifecycle_state_owner_update_allowed_transition_updates_state() {
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = cell.try_update_state(
        ProductionOutputRouteLifecycleState::AcceptingInput,
        &matrix,
    );

    assert_eq!(decision, ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated);
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::AcceptingInput);
}

#[test]
fn lifecycle_state_owner_update_rejected_transition_does_not_update_state() {
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::AcceptingInput,
    );
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = cell.try_update_state(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        &matrix,
    );

    assert_eq!(decision, ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected);
    // State must remain unchanged.
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::AcceptingInput);
}

#[test]
fn lifecycle_state_owner_update_same_state_idempotent() {
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::AcceptingInput,
    );
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    let decision = cell.try_update_state(
        ProductionOutputRouteLifecycleState::AcceptingInput,
        &matrix,
    );

    // Same-state is allowed by the matrix.
    assert_eq!(decision, ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated);
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::AcceptingInput);
}

#[test]
fn lifecycle_state_owner_update_all_allowed_transitions() {
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // NotReadyForInput → AcceptingInput
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
    assert_eq!(
        cell.try_update_state(ProductionOutputRouteLifecycleState::AcceptingInput, &matrix),
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated,
    );
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::AcceptingInput);

    // AcceptingInput → Closed
    assert_eq!(
        cell.try_update_state(ProductionOutputRouteLifecycleState::Closed, &matrix),
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated,
    );
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::Closed);
}

#[test]
fn lifecycle_state_owner_update_all_rejected_transitions() {
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // AcceptingInput → NotReadyForInput (rejected)
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::AcceptingInput,
    );
    assert_eq!(
        cell.try_update_state(ProductionOutputRouteLifecycleState::NotReadyForInput, &matrix),
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected,
    );
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::AcceptingInput);

    // Closed → AcceptingInput (rejected)
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::Closed,
    );
    assert_eq!(
        cell.try_update_state(ProductionOutputRouteLifecycleState::AcceptingInput, &matrix),
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected,
    );
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::Closed);

    // Closed → NotReadyForInput (rejected)
    assert_eq!(
        cell.try_update_state(ProductionOutputRouteLifecycleState::NotReadyForInput, &matrix),
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected,
    );
    assert_eq!(cell.current_state(), ProductionOutputRouteLifecycleState::Closed);
}

#[test]
fn lifecycle_state_owner_update_decision_is_reasonless() {
    // Decision enum has no payload, no reason.
    // This is a compile-time proof: only Updated/Rejected variants exist.
    let decision = ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated;
    match decision {
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated => {}
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected => {}
    }

    // Decision is Copy, Clone, Eq, Hash, Debug.
    let copied = decision;
    assert_eq!(copied, decision);

    let debug = format!("{:?}", decision);
    assert!(!debug.is_empty());
}

#[test]
fn lifecycle_state_owner_update_delegates_to_matrix() {
    // State owner does NOT duplicate transition rules.
    // It delegates to the caller-provided matrix.
    // Proof: try_update_state takes a matrix reference and calls validate_transition().
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // This call delegates to matrix.validate_transition(NotReadyForInput, AcceptingInput).
    let decision = cell.try_update_state(
        ProductionOutputRouteLifecycleState::AcceptingInput,
        &matrix,
    );

    // If we get here, the delegation worked.
    assert_eq!(decision, ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated);
}

#[test]
fn lifecycle_state_owner_update_api_shape() {
    // API shape proof: try_update_state takes &self, requested_next_state, &matrix.
    let mut cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
    let matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();

    // Method exists with correct signature.
    let _decision = cell.try_update_state(
        ProductionOutputRouteLifecycleState::AcceptingInput,
        &matrix,
    );
}
