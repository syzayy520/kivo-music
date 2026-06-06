/// Pure value contract for route closed reasons.
///
/// Each variant is a caller-supplied label only.
/// Not a lifecycle state holder. Not a close authority.
/// Does not check current open/closed state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProductionOutputRouteRouteClosedReason {
    /// Route was never opened.
    NotOpened,
    /// Closed because a future caller explicitly supplied close reason.
    ExplicitClose,
    /// Attempted operation on an already-closed route.
    AlreadyClosed,
    /// Rejected after route was closed.
    RejectedAfterClose,
}
