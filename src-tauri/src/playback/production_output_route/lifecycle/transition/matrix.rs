use super::decision::ProductionOutputRouteLifecycleTransitionDecision;
use super::super::state::ProductionOutputRouteLifecycleState;

/// Stateless lifecycle transition validation matrix.
///
/// Receives caller-supplied current and requested next state.
/// Returns Allowed/Re jected decision.
/// Does not hold state, mutate state, or perform any runtime action.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteLifecycleTransitionMatrix;

#[allow(dead_code)]
impl ProductionOutputRouteLifecycleTransitionMatrix {
    pub(crate) fn new() -> Self {
        Self
    }

    /// Validate whether the requested transition is allowed.
    ///
    /// Allowed transitions:
    /// - NotReadyForInput → NotReadyForInput (idempotent)
    /// - NotReadyForInput → AcceptingInput
    /// - NotReadyForInput → Closed
    /// - AcceptingInput → AcceptingInput (idempotent)
    /// - AcceptingInput → Closed
    /// - Closed → Closed (idempotent)
    ///
    /// Rejected transitions:
    /// - AcceptingInput → NotReadyForInput (cannot go backward)
    /// - Closed → AcceptingInput (cannot reopen)
    /// - Closed → NotReadyForInput (cannot reopen)
    pub(crate) fn validate_transition(
        &self,
        current_state: ProductionOutputRouteLifecycleState,
        requested_next_state: ProductionOutputRouteLifecycleState,
    ) -> ProductionOutputRouteLifecycleTransitionDecision {
        use ProductionOutputRouteLifecycleState as S;

        match (current_state, requested_next_state) {
            (S::NotReadyForInput, S::NotReadyForInput)
            | (S::NotReadyForInput, S::AcceptingInput)
            | (S::NotReadyForInput, S::Closed)
            | (S::AcceptingInput, S::AcceptingInput)
            | (S::AcceptingInput, S::Closed)
            | (S::Closed, S::Closed) => {
                ProductionOutputRouteLifecycleTransitionDecision::Allowed
            }
            (S::AcceptingInput, S::NotReadyForInput)
            | (S::Closed, S::AcceptingInput)
            | (S::Closed, S::NotReadyForInput) => {
                ProductionOutputRouteLifecycleTransitionDecision::Rejected
            }
        }
    }
}
