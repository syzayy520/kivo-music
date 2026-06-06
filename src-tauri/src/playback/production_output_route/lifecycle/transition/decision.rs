/// Transition validation result.
///
/// Pure value — no reason payload, no state, no failure.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteLifecycleTransitionDecision {
    /// The requested transition is valid under the matrix.
    Allowed,
    /// The requested transition is invalid under the matrix.
    /// Intentionally reasonless — no payload, no current/next state.
    Rejected,
}
