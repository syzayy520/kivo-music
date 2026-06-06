#![allow(dead_code, unused_imports)]

mod failure;
mod input;

pub(crate) use failure::{
    ProductionOutputRouteBackpressure, ProductionOutputRouteFailure,
    ProductionOutputRouteFailureClass, ProductionOutputRouteFormatDescriptor,
    ProductionOutputRouteFormatMismatch, ProductionOutputRouteRouteClosed,
    ProductionOutputRouteRouteClosedReason, ProductionOutputRouteSinkFailure,
    ProductionOutputRouteStreamFormat, ProductionOutputRouteUnderrun,
};
pub(crate) use input::ProductionOutputRouteFrameInput;

#[cfg(test)]
mod tests;
