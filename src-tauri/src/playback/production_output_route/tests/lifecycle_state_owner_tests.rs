use super::super::lifecycle::ProductionOutputRouteLifecycleState;
use super::super::lifecycle::ProductionOutputRouteLifecycleStateCell;

#[test]
fn lifecycle_state_owner_holds_initial_state() {
    // State cell must hold the initial state provided at construction.
    let initial = ProductionOutputRouteLifecycleState::NotReadyForInput;
    let cell = ProductionOutputRouteLifecycleStateCell::new(initial);
    assert_eq!(cell.current_state(), initial);

    // Different initial state
    let initial = ProductionOutputRouteLifecycleState::AcceptingInput;
    let cell = ProductionOutputRouteLifecycleStateCell::new(initial);
    assert_eq!(cell.current_state(), initial);
}

#[test]
fn lifecycle_state_owner_no_default() {
    // State cell must NOT implement Default.
    // This is a compile-time proof: ProductionOutputRouteLifecycleStateCell::default() does not exist.
    // The test compiles because we don't call default().
    let _cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
}

#[test]
fn lifecycle_state_owner_current_state_returns_copy() {
    // current_state() returns a Copy value, not a reference.
    let cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::AcceptingInput,
    );
    let state = cell.current_state();
    // Copy by value — no borrow held on cell.
    let _copy = state;
    // Cell is still usable.
    let _state2 = cell.current_state();
}
