use super::super::config::input_acceptance::ProductionOutputRouteInputAcceptancePolicy;
use super::super::failure::ProductionOutputRouteFailure;
use super::super::input::ProductionOutputRouteFrameInput;

/// Contract-level config validation authority.
///
/// Owns an `ProductionOutputRouteInputAcceptancePolicy` and delegates
/// acceptance to the existing config/input_acceptance contract.
///
/// Does not store or expose `ProductionOutputRouteIdentity`.
/// Does not retain input/frame/sample data.
/// Does not maintain mutable pending_frames state.
#[derive(Clone, Debug)]
pub(crate) struct ProductionOutputRouteConfigAuthority {
    policy: ProductionOutputRouteInputAcceptancePolicy,
}

#[allow(dead_code)]
impl ProductionOutputRouteConfigAuthority {
    /// Create a new config authority wrapping the given acceptance policy.
    pub(crate) fn new(policy: ProductionOutputRouteInputAcceptancePolicy) -> Self {
        Self { policy }
    }

    /// Delegate input acceptance to the existing policy.
    ///
    /// `pending_frames` is an explicit argument, not retained state.
    /// Returns `Ok(())` if accepted, or the existing failure taxonomy on rejection.
    pub(crate) fn accept(
        &self,
        input: ProductionOutputRouteFrameInput<'_>,
        pending_frames: usize,
    ) -> Result<(), ProductionOutputRouteFailure> {
        self.policy.accept(input, pending_frames)
    }
}
