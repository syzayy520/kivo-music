use super::super::failure::ProductionOutputRouteFailure;
use super::super::input::ProductionOutputRouteFrameInput;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) enum ProductionOutputRouteInputAdmissionResult<'a> {
    Allowed(ProductionOutputRouteFrameInput<'a>),
    Rejected(ProductionOutputRouteFailure),
}