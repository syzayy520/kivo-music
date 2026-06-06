pub(crate) mod config;
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

#[allow(dead_code)]
fn production_output_route_contract_lint_anchor(frame: &crate::playback::output::AudioOutputFrame) {
    let input = ProductionOutputRouteFrameInput::from_frame(frame);
    let underrun = ProductionOutputRouteUnderrun::new(1, 0);
    let sink_failure = ProductionOutputRouteSinkFailure::new("contract", "unattached");
    let route_closed =
        ProductionOutputRouteRouteClosed::new(ProductionOutputRouteRouteClosedReason::NotOpened);
    let classes: [ProductionOutputRouteFailureClass; 3] = [
        ProductionOutputRouteFailure::from(underrun).class(),
        ProductionOutputRouteFailure::from(sink_failure.clone()).class(),
        ProductionOutputRouteFailure::from(route_closed).class(),
    ];

    let _ = (
        input.position_ms(),
        input.sample_count(),
        underrun.requested_frames(),
        underrun.available_frames(),
        underrun.missing_frames(),
        sink_failure.operation(),
        sink_failure.message(),
        route_closed.reason(),
        classes,
        ProductionOutputRouteRouteClosedReason::ClosedByOwner,
    );
}

#[cfg(test)]
mod tests;
