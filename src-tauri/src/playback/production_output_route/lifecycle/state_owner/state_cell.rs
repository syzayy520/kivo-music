use super::super::state::ProductionOutputRouteLifecycleState;

/// Lifecycle state owner: holds the current lifecycle state.
///
/// Not a product PlaybackState holder. Not a runtime owner.
/// Pure route contract state cell only.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteLifecycleStateCell {
    current_state: ProductionOutputRouteLifecycleState,
}

#[allow(dead_code)]
impl ProductionOutputRouteLifecycleStateCell {
    /// Create a new state cell with the given initial state.
    ///
    /// No Default, no fallback, no implicit NotReadyForInput.
    /// Caller must explicitly provide the initial state.
    pub(crate) fn new(initial_state: ProductionOutputRouteLifecycleState) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    /// Get the current lifecycle state.
    ///
    /// Returns a copy (Copy type) — no borrow held.
    pub(crate) fn current_state(&self) -> ProductionOutputRouteLifecycleState {
        self.current_state
    }

    /// Replace the current state after a validated transition.
    ///
    /// This is a pub(super) helper that allows cross-file update from `update.rs`
    /// while keeping the `current_state` field private.
    ///
    /// # Safety Contract
    ///
    /// Caller must ensure the transition has been validated by a transition matrix
    /// before calling this method. This method does NOT validate — it only updates.
    pub(super) fn replace_current_state_after_validated_transition(
        &mut self,
        next_state: ProductionOutputRouteLifecycleState,
    ) {
        self.current_state = next_state;
    }
}
