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
    let descriptor = ProductionOutputRouteFormatDescriptor::from_stream(input.stream());
    let mismatch = ProductionOutputRouteFormatMismatch::new(descriptor, descriptor);
    let backpressure = ProductionOutputRouteBackpressure::new(1, 1);
    let underrun = ProductionOutputRouteUnderrun::new(1, 0);
    let sink_failure = ProductionOutputRouteSinkFailure::new("contract", "unattached");
    let route_closed =
        ProductionOutputRouteRouteClosed::new(ProductionOutputRouteRouteClosedReason::NotOpened);
    let classes: [ProductionOutputRouteFailureClass; 5] = [
        ProductionOutputRouteFailure::from(backpressure).class(),
        ProductionOutputRouteFailure::from(underrun).class(),
        ProductionOutputRouteFailure::from(sink_failure.clone()).class(),
        ProductionOutputRouteFailure::from(route_closed).class(),
        ProductionOutputRouteFailure::from(mismatch).class(),
    ];

    let _ = (
        input.position_ms(),
        input.sample_count(),
        descriptor.sample_rate_hz(),
        descriptor.channels(),
        descriptor.sample_format(),
        mismatch.expected(),
        mismatch.actual(),
        mismatch.is_mismatch(),
        backpressure.pending_frames(),
        backpressure.capacity_frames(),
        backpressure.is_capacity_reached(),
        underrun.requested_frames(),
        underrun.available_frames(),
        underrun.missing_frames(),
        sink_failure.operation(),
        sink_failure.message(),
        route_closed.reason(),
        classes,
        ProductionOutputRouteRouteClosedReason::ClosedByOwner,
        ProductionOutputRouteStreamFormat::Float32,
        ProductionOutputRouteStreamFormat::Signed16,
        ProductionOutputRouteStreamFormat::Signed24,
        ProductionOutputRouteStreamFormat::Signed32,
    );
}

#[cfg(test)]
mod tests;
