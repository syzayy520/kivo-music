use super::state::ProductionOutputRouteLifecycleState;
use crate::playback::production_output_route::failure::{
    ProductionOutputRouteFailure, ProductionOutputRouteRouteClosed,
    ProductionOutputRouteRouteClosedReason,
};
use crate::playback::production_output_route::input::ProductionOutputRouteFrameInput;

/// Stateless lifecycle input gate.
///
/// Receives caller-supplied lifecycle state and frame input.
/// Maps state to allow/reject decision.
/// Does not hold state, mutate state, check format, check capacity,
/// call config authority, or inspect sample contents.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteLifecycleInputGate;

#[allow(dead_code)]
impl ProductionOutputRouteLifecycleInputGate {
    pub(crate) fn new() -> Self {
        Self
    }

    /// Gate input based on caller-supplied lifecycle state.
    ///
    /// - AcceptingInput: returns Ok(input), pass-through by value.
    /// - NotReadyForInput: returns Err(RouteClosed(NotOpened)).
    /// - Closed: returns Err(RouteClosed(RejectedAfterClose)).
    pub(crate) fn gate_input<'a>(
        &self,
        state: ProductionOutputRouteLifecycleState,
        input: ProductionOutputRouteFrameInput<'a>,
    ) -> Result<ProductionOutputRouteFrameInput<'a>, ProductionOutputRouteFailure> {
        match state {
            ProductionOutputRouteLifecycleState::AcceptingInput => Ok(input),
            ProductionOutputRouteLifecycleState::NotReadyForInput => Err(
                ProductionOutputRouteFailure::from(ProductionOutputRouteRouteClosed::new(
                    ProductionOutputRouteRouteClosedReason::NotOpened,
                )),
            ),
            ProductionOutputRouteLifecycleState::Closed => Err(ProductionOutputRouteFailure::from(
                ProductionOutputRouteRouteClosed::new(
                    ProductionOutputRouteRouteClosedReason::RejectedAfterClose,
                ),
            )),
        }
    }
}
