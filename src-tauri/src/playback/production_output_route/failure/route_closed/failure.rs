use super::reason::ProductionOutputRouteRouteClosedReason;

/// Failure wrapper that only wraps a reason.
///
/// Does not perform state transitions.
/// Does not close anything.
/// Does not track open/closed state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteRouteClosed {
    reason: ProductionOutputRouteRouteClosedReason,
}

impl ProductionOutputRouteRouteClosed {
    /// Create a new route closed failure with the given reason.
    #[allow(dead_code)]
    pub(crate) fn new(reason: ProductionOutputRouteRouteClosedReason) -> Self {
        Self { reason }
    }

    /// Returns the reason by value.
    #[allow(dead_code)]
    pub(crate) fn reason(&self) -> ProductionOutputRouteRouteClosedReason {
        self.reason
    }
}
