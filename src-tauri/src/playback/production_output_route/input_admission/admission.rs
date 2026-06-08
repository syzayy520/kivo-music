use super::super::authority::ProductionOutputRouteConfigAuthority;
use super::super::input::ProductionOutputRouteFrameInput;
use super::super::lifecycle::ProductionOutputRouteLifecycleInputGate;
use super::super::lifecycle::ProductionOutputRouteLifecycleState;
use super::result::ProductionOutputRouteInputAdmissionResult;

/// Input admission authority: chains lifecycle gate → config authority in fixed order.
///
/// Stateless. Zero-sized. No retained state.
/// No StateCell ownership. No TransitionMatrix ownership.
/// No close history. No runtime handle.
/// No OutputSink / NativePipeline dependency.
/// No product state.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteInputAdmission;

#[allow(dead_code)]
impl ProductionOutputRouteInputAdmission {
    /// Admit input through lifecycle gate then config authority.
    ///
    /// Fixed order: lifecycle gate first, config authority second.
    /// If lifecycle gate rejects, returns Rejected(failure) without calling config authority.
    /// If lifecycle gate accepts, delegates to config authority.
    /// Preserves original failure exactly from whichever authority rejected.
    ///
    /// # Arguments
    ///
    /// * `lifecycle_gate` - Caller-provided lifecycle input gate.
    /// * `config_authority` - Caller-provided config authority.
    /// * `state` - Caller-provided lifecycle state.
    /// * `input` - Frame input to admit (Copy by value).
    /// * `pending_frames` - Pending frames count for config authority.
    ///
    /// # Returns
    ///
    /// * `ProductionOutputRouteInputAdmissionResult<'a>` — Allowed(input) or Rejected(failure).
    pub(crate) fn admit_input<'a>(
        &self,
        lifecycle_gate: &ProductionOutputRouteLifecycleInputGate,
        config_authority: &ProductionOutputRouteConfigAuthority,
        state: ProductionOutputRouteLifecycleState,
        input: ProductionOutputRouteFrameInput<'a>,
        pending_frames: usize,
    ) -> ProductionOutputRouteInputAdmissionResult<'a> {
        // Step 1: Lifecycle gate
        match lifecycle_gate.gate_input(state, input) {
            Ok(input) => {
                // Step 2: Config authority
                match config_authority.accept(input, pending_frames) {
                    Ok(()) => ProductionOutputRouteInputAdmissionResult::Allowed(input),
                    Err(failure) => ProductionOutputRouteInputAdmissionResult::Rejected(failure),
                }
            }
            Err(failure) => ProductionOutputRouteInputAdmissionResult::Rejected(failure),
        }
    }
}
