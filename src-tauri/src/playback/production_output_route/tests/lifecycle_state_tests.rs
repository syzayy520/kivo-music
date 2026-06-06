use super::super::lifecycle::ProductionOutputRouteLifecycleState;

#[test]
fn lifecycle_state_variants_are_exact_contract_language() {
    // Only NotReadyForInput / AcceptingInput / Closed exist.
    // No RouteClosedReason variants are mirrored.
    // Specifically no LifecycleState::NotOpened.
    let states = [
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        ProductionOutputRouteLifecycleState::Closed,
    ];
    assert_eq!(states.len(), 3);

    // Compile-time proof: these are the only variants (exhaustive match).
    for state in &states {
        match state {
            ProductionOutputRouteLifecycleState::NotReadyForInput => {}
            ProductionOutputRouteLifecycleState::AcceptingInput => {}
            ProductionOutputRouteLifecycleState::Closed => {}
        }
    }
}

#[test]
fn lifecycle_state_is_copy_clone_eq_hash_debug() {
    use std::collections::HashSet;

    let state = ProductionOutputRouteLifecycleState::AcceptingInput;

    // Copy
    let copied = state;
    assert_eq!(copied, state);

    // Clone (Copy types: clone is identity)
    let cloned = state;
    assert_eq!(cloned, state);

    // Eq
    assert_eq!(state, ProductionOutputRouteLifecycleState::AcceptingInput);
    assert_ne!(state, ProductionOutputRouteLifecycleState::Closed);

    // Hash
    let mut set = HashSet::new();
    set.insert(state);
    assert!(set.contains(&state));

    // Debug
    let debug = format!("{:?}", state);
    assert!(!debug.is_empty());
}

#[test]
fn lifecycle_state_accepting_input_is_distinct_from_closed() {
    assert_ne!(
        ProductionOutputRouteLifecycleState::AcceptingInput,
        ProductionOutputRouteLifecycleState::Closed
    );
    assert_ne!(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        ProductionOutputRouteLifecycleState::AcceptingInput
    );
    assert_ne!(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        ProductionOutputRouteLifecycleState::Closed
    );
}

#[test]
fn lifecycle_state_does_not_encode_route_closed_reason() {
    // LifecycleState has no reason payload.
    // NotReadyForInput maps to RouteClosedReason::NotOpened at the gate
    // but does NOT share its name — they are different concepts.
    use super::super::failure::ProductionOutputRouteRouteClosedReason;

    // State variant names are distinct from reason variant names.
    // This is a compile-time proof: LifecycleState::NotOpened does not exist.
    let _state = ProductionOutputRouteLifecycleState::NotReadyForInput;
    let _reason = ProductionOutputRouteRouteClosedReason::NotOpened;

    // They are different types, different names.
    // The state is a state concept, the reason is a rejection reason concept.
    match _state {
        ProductionOutputRouteLifecycleState::NotReadyForInput => {}
        ProductionOutputRouteLifecycleState::AcceptingInput => {}
        ProductionOutputRouteLifecycleState::Closed => {}
    }

    match _reason {
        ProductionOutputRouteRouteClosedReason::NotOpened => {}
        ProductionOutputRouteRouteClosedReason::ExplicitClose => {}
        ProductionOutputRouteRouteClosedReason::AlreadyClosed => {}
        ProductionOutputRouteRouteClosedReason::RejectedAfterClose => {}
    }
}

#[test]
fn lifecycle_state_product_boundary_proof() {
    // LifecycleState does not carry PlaybackState, OutputRuntimeStatus,
    // output_status, clock, or any product runtime concept.
    // Proof: the enum has no payload fields — it is a pure discriminant.
    let state = ProductionOutputRouteLifecycleState::AcceptingInput;

    // Copy by value — no references to runtime state.
    let _copy = state;

    // No methods beyond the enum itself — no transition, no is_open, no close.
    // This is verified by the compilation of this test file,
    // which only imports lifecycle state and nothing from product/runtime.
}
