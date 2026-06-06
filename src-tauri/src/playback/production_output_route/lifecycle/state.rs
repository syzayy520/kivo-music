/// Route lifecycle contract state.
///
/// Not a product PlaybackState. Not a runtime owner state.
/// Not a sink/native pipeline state. Pure route contract language only.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteLifecycleState {
    /// Route is not yet ready to accept input.
    /// Maps to RouteClosed(NotOpened) at the gate.
    NotReadyForInput,
    /// Route accepts input — input may proceed to config authority.
    /// Does not mean output sink is open or real playback is running.
    AcceptingInput,
    /// Route no longer accepts input.
    /// Maps to RouteClosed(RejectedAfterClose) at the gate.
    Closed,
}
