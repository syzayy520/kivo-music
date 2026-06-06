#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteRouteClosedReason {
    NotOpened,
    ClosedByOwner,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteRouteClosed {
    reason: ProductionOutputRouteRouteClosedReason,
}

impl ProductionOutputRouteRouteClosed {
    pub(crate) fn new(reason: ProductionOutputRouteRouteClosedReason) -> Self {
        Self { reason }
    }

    pub(crate) fn reason(&self) -> ProductionOutputRouteRouteClosedReason {
        self.reason
    }
}
