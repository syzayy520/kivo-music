/// Lifecycle close authority decision.
///
/// Reasonless — no payload, no reason. Only close-specific decisions.
/// Not a RouteClosedReason. Not a ProductionOutputRouteFailure.
/// Not a runtime close result. Not an OutputSink close result.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteLifecycleCloseDecision {
    /// Close request was accepted at lifecycle contract layer.
    /// State was updated to ProductionOutputRouteLifecycleState::Closed.
    /// This is not runtime close success. Not OutputSink close success.
    Closed,
    /// Current state was already ProductionOutputRouteLifecycleState::Closed.
    /// request_close returned without calling try_update_state.
    /// This is close-specific decision, not RouteClosedReason::AlreadyClosed.
    AlreadyClosed,
    /// state_owner.try_update_state(Closed, matrix) returned Rejected.
    /// State remains unchanged. Reasonless.
    /// Currently may be unreachable under existing P0-112 matrix.
    Rejected,
}
