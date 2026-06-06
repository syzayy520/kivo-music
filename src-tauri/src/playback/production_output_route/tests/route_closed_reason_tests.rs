use super::super::{
    ProductionOutputRouteFailure, ProductionOutputRouteFailureClass,
    ProductionOutputRouteRouteClosed, ProductionOutputRouteRouteClosedReason,
};

#[test]
fn route_closed_reason_variants_are_explicit_values() {
    let variants = [
        ProductionOutputRouteRouteClosedReason::NotOpened,
        ProductionOutputRouteRouteClosedReason::ExplicitClose,
        ProductionOutputRouteRouteClosedReason::AlreadyClosed,
        ProductionOutputRouteRouteClosedReason::RejectedAfterClose,
    ];

    // All variants are distinct
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn route_closed_reason_not_opened_is_value_only() {
    let reason = ProductionOutputRouteRouteClosedReason::NotOpened;

    // Pure value: Debug, Copy, Clone, Eq, Hash
    let _debug = format!("{:?}", reason);
    let copied = reason;
    let cloned = reason.clone();
    assert_eq!(reason, copied);
    assert_eq!(reason, cloned);
}

#[test]
fn route_closed_reason_explicit_close_is_value_only() {
    let reason = ProductionOutputRouteRouteClosedReason::ExplicitClose;

    let _debug = format!("{:?}", reason);
    let copied = reason;
    let cloned = reason.clone();
    assert_eq!(reason, copied);
    assert_eq!(reason, cloned);
}

#[test]
fn route_closed_reason_already_closed_is_value_only() {
    let reason = ProductionOutputRouteRouteClosedReason::AlreadyClosed;

    let _debug = format!("{:?}", reason);
    let copied = reason;
    let cloned = reason.clone();
    assert_eq!(reason, copied);
    assert_eq!(reason, cloned);
}

#[test]
fn route_closed_reason_rejected_after_close_is_value_only() {
    let reason = ProductionOutputRouteRouteClosedReason::RejectedAfterClose;

    let _debug = format!("{:?}", reason);
    let copied = reason;
    let cloned = reason.clone();
    assert_eq!(reason, copied);
    assert_eq!(reason, cloned);
}

#[test]
fn route_closed_failure_wraps_reason() {
    let reason = ProductionOutputRouteRouteClosedReason::ExplicitClose;
    let failure = ProductionOutputRouteRouteClosed::new(reason);

    assert_eq!(failure.reason(), reason);
}

#[test]
fn route_closed_failure_preserves_exact_reason() {
    let reasons = [
        ProductionOutputRouteRouteClosedReason::NotOpened,
        ProductionOutputRouteRouteClosedReason::ExplicitClose,
        ProductionOutputRouteRouteClosedReason::AlreadyClosed,
        ProductionOutputRouteRouteClosedReason::RejectedAfterClose,
    ];

    for reason in reasons {
        let failure = ProductionOutputRouteRouteClosed::new(reason);
        assert_eq!(failure.reason(), reason);
    }
}

#[test]
fn route_closed_failure_classifies_as_route_closed() {
    let failure = ProductionOutputRouteRouteClosed::new(
        ProductionOutputRouteRouteClosedReason::ExplicitClose,
    );
    let route_failure = ProductionOutputRouteFailure::from(failure);

    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::RouteClosed
    );
}

#[test]
fn route_closed_failure_converts_into_existing_failure_taxonomy() {
    let reasons = [
        ProductionOutputRouteRouteClosedReason::NotOpened,
        ProductionOutputRouteRouteClosedReason::ExplicitClose,
        ProductionOutputRouteRouteClosedReason::AlreadyClosed,
        ProductionOutputRouteRouteClosedReason::RejectedAfterClose,
    ];

    for reason in reasons {
        let failure = ProductionOutputRouteRouteClosed::new(reason);
        let route_failure = ProductionOutputRouteFailure::from(failure);

        assert_eq!(
            route_failure.class(),
            ProductionOutputRouteFailureClass::RouteClosed
        );
    }
}

#[test]
fn route_closed_reason_does_not_require_lifecycle_state() {
    // RouteClosedReason is a pure value enum.
    // It can be constructed without any runtime/lifecycle state.
    let _ = ProductionOutputRouteRouteClosedReason::NotOpened;
    let _ = ProductionOutputRouteRouteClosedReason::ExplicitClose;
    let _ = ProductionOutputRouteRouteClosedReason::AlreadyClosed;
    let _ = ProductionOutputRouteRouteClosedReason::RejectedAfterClose;
}
