use super::super::transition::ProductionOutputRouteLifecycleTransitionDecision;
use super::super::transition::ProductionOutputRouteLifecycleTransitionMatrix;
use super::super::ProductionOutputRouteLifecycleState;
use super::state_cell::ProductionOutputRouteLifecycleStateCell;

/// Decision result for lifecycle state owner update.
///
/// Reasonless — no payload, no reason. Only Updated or Rejected.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteLifecycleStateOwnerUpdateDecision {
    /// Transition was allowed by the matrix; state was updated.
    Updated,
    /// Transition was rejected by the matrix; state was not changed.
    Rejected,
}

#[allow(dead_code)]
impl ProductionOutputRouteLifecycleStateCell {
    /// Try to update the current state to the requested next state.
    ///
    /// Delegates to the caller-provided transition matrix for validation.
    /// Does NOT duplicate transition rules — must use matrix.validate_transition().
    ///
    /// # Arguments
    ///
    /// * `requested_next_state` - The state to transition to.
    /// * `transition_matrix` - The caller-provided transition matrix for validation.
    ///
    /// # Returns
    ///
    /// * `Updated` if the transition was allowed and state was updated.
    /// * `Rejected` if the transition was rejected (state unchanged).
    pub(crate) fn try_update_state(
        &mut self,
        requested_next_state: ProductionOutputRouteLifecycleState,
        transition_matrix: &ProductionOutputRouteLifecycleTransitionMatrix,
    ) -> ProductionOutputRouteLifecycleStateOwnerUpdateDecision {
        let current = self.current_state();
        let decision = transition_matrix.validate_transition(current, requested_next_state);

        match decision {
            ProductionOutputRouteLifecycleTransitionDecision::Allowed => {
                self.replace_current_state_after_validated_transition(requested_next_state);
                ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated
            }
            ProductionOutputRouteLifecycleTransitionDecision::Rejected => {
                ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected
            }
        }
    }
}
