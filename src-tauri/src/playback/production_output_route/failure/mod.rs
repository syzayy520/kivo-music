mod backpressure;
mod format_mismatch;
mod route_closed;
mod sink_failure;
mod underrun;

pub(crate) use backpressure::ProductionOutputRouteBackpressure;
pub(crate) use format_mismatch::{
    ProductionOutputRouteFormatDescriptor, ProductionOutputRouteFormatMismatch,
    ProductionOutputRouteStreamFormat,
};
pub(crate) use route_closed::{
    ProductionOutputRouteRouteClosed, ProductionOutputRouteRouteClosedReason,
};
pub(crate) use sink_failure::ProductionOutputRouteSinkFailure;
pub(crate) use underrun::ProductionOutputRouteUnderrun;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteFailureClass {
    Backpressure,
    Underrun,
    SinkFailure,
    RouteClosed,
    FormatMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProductionOutputRouteFailure {
    Backpressure(ProductionOutputRouteBackpressure),
    Underrun(ProductionOutputRouteUnderrun),
    SinkFailure(ProductionOutputRouteSinkFailure),
    RouteClosed(ProductionOutputRouteRouteClosed),
    FormatMismatch(ProductionOutputRouteFormatMismatch),
}

impl ProductionOutputRouteFailure {
    pub(crate) fn class(&self) -> ProductionOutputRouteFailureClass {
        match self {
            Self::Backpressure(_) => ProductionOutputRouteFailureClass::Backpressure,
            Self::Underrun(_) => ProductionOutputRouteFailureClass::Underrun,
            Self::SinkFailure(_) => ProductionOutputRouteFailureClass::SinkFailure,
            Self::RouteClosed(_) => ProductionOutputRouteFailureClass::RouteClosed,
            Self::FormatMismatch(_) => ProductionOutputRouteFailureClass::FormatMismatch,
        }
    }
}

impl From<ProductionOutputRouteBackpressure> for ProductionOutputRouteFailure {
    fn from(failure: ProductionOutputRouteBackpressure) -> Self {
        Self::Backpressure(failure)
    }
}

impl From<ProductionOutputRouteUnderrun> for ProductionOutputRouteFailure {
    fn from(failure: ProductionOutputRouteUnderrun) -> Self {
        Self::Underrun(failure)
    }
}

impl From<ProductionOutputRouteSinkFailure> for ProductionOutputRouteFailure {
    fn from(failure: ProductionOutputRouteSinkFailure) -> Self {
        Self::SinkFailure(failure)
    }
}

impl From<ProductionOutputRouteRouteClosed> for ProductionOutputRouteFailure {
    fn from(failure: ProductionOutputRouteRouteClosed) -> Self {
        Self::RouteClosed(failure)
    }
}

impl From<ProductionOutputRouteFormatMismatch> for ProductionOutputRouteFailure {
    fn from(failure: ProductionOutputRouteFormatMismatch) -> Self {
        Self::FormatMismatch(failure)
    }
}
