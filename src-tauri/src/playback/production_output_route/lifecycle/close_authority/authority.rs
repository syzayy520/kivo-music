use super::super::transition::ProductionOutputRouteLifecycleTransitionMatrix;
use super::super::ProductionOutputRouteLifecycleState;
use super::super::ProductionOutputRouteLifecycleStateCell;
use super::super::ProductionOutputRouteLifecycleStateOwnerUpdateDecision;
use super::decision::ProductionOutputRouteLifecycleCloseDecision;

/// Lifecycle close authority: evaluates close requests at lifecycle contract layer.
///
/// Stateless. Zero-sized. No retained state.
/// No StateCell ownership. No TransitionMatrix ownership.
/// No close history. No runtime handle.
/// No OutputSink / NativePipeline dependency.
/// No product state.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteLifecycleCloseAuthority;

#[allow(dead_code)]
impl ProductionOutputRouteLifecycleCloseAuthority {
    /// Request close at lifecycle contract layer.
    ///
    /// Reads current_state before any update.
    /// If current_state is AlreadyClosed, returns AlreadyClosed without calling try_update_state.
    /// Otherwise delegates to state_cell.try_update_state(Closed, matrix).
    ///
    /// # Arguments
    ///
    /// * `state_cell` - Caller-provided mutable lifecycle state cell.
    /// * `transition_matrix` - Caller-provided transition matrix for validation.
    ///
    /// # Returns
    ///
    /// * `ProductionOutputRouteLifecycleCloseDecision` — reasonless close decision.
    pub(crate) fn request_close(
        &self,
        state_cell: &mut ProductionOutputRouteLifecycleStateCell,
        transition_matrix: &ProductionOutputRouteLifecycleTransitionMatrix,
    ) -> ProductionOutputRouteLifecycleCloseDecision {
        let current_state = state_cell.current_state();

        // AlreadyClosed branch: current_state is already Closed.
        // Return before calling try_update_state.
        if current_state == ProductionOutputRouteLifecycleState::Closed {
            return ProductionOutputRouteLifecycleCloseDecision::AlreadyClosed;
        }

        // Non-Closed states: attempt transition to Closed.
        match state_cell.try_update_state(
            ProductionOutputRouteLifecycleState::Closed,
            transition_matrix,
        ) {
            ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Updated => {
                ProductionOutputRouteLifecycleCloseDecision::Closed
            }
            ProductionOutputRouteLifecycleStateOwnerUpdateDecision::Rejected => {
                ProductionOutputRouteLifecycleCloseDecision::Rejected
            }
        }
    }
}
