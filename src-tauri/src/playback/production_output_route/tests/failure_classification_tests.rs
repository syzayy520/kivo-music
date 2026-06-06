use std::collections::HashSet;

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

use super::super::{
    ProductionOutputRouteBackpressure, ProductionOutputRouteFailure,
    ProductionOutputRouteFailureClass, ProductionOutputRouteFormatDescriptor,
    ProductionOutputRouteFormatMismatch, ProductionOutputRouteRouteClosed,
    ProductionOutputRouteRouteClosedReason, ProductionOutputRouteSinkFailure,
    ProductionOutputRouteStreamFormat, ProductionOutputRouteUnderrun,
};

fn stream(sample_rate_hz: u32, channels: u16, sample_format: AudioSampleFormat) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format,
    }
}

#[test]
fn production_output_route_failure_backpressure_has_capacity_contract() {
    let failure = ProductionOutputRouteBackpressure::new(8, 8);
    let route_failure = ProductionOutputRouteFailure::from(failure);

    assert_eq!(failure.pending_frames(), 8);
    assert_eq!(failure.capacity_frames(), 8);
    assert!(failure.is_capacity_reached());
    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::Backpressure
    );
}

#[test]
fn production_output_route_failure_underrun_has_missing_frame_contract() {
    let failure = ProductionOutputRouteUnderrun::new(512, 128);
    let route_failure = ProductionOutputRouteFailure::from(failure);

    assert_eq!(failure.requested_frames(), 512);
    assert_eq!(failure.available_frames(), 128);
    assert_eq!(failure.missing_frames(), 384);
    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::Underrun
    );
}

#[test]
fn production_output_route_failure_sink_failure_has_operation_message_contract() {
    let failure = ProductionOutputRouteSinkFailure::new("submit_frame", "sink rejected frame");
    let route_failure = ProductionOutputRouteFailure::from(failure.clone());

    assert_eq!(failure.operation(), "submit_frame");
    assert_eq!(failure.message(), "sink rejected frame");
    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::SinkFailure
    );
}

#[test]
fn production_output_route_failure_route_closed_has_reason_contract() {
    let failure = ProductionOutputRouteRouteClosed::new(
        ProductionOutputRouteRouteClosedReason::ClosedByOwner,
    );
    let route_failure = ProductionOutputRouteFailure::from(failure);

    assert_eq!(
        failure.reason(),
        ProductionOutputRouteRouteClosedReason::ClosedByOwner
    );
    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::RouteClosed
    );
    assert_ne!(
        ProductionOutputRouteRouteClosedReason::ClosedByOwner,
        ProductionOutputRouteRouteClosedReason::NotOpened
    );
}

#[test]
fn production_output_route_failure_format_mismatch_has_descriptor_contract() {
    let expected = ProductionOutputRouteFormatDescriptor::from_stream(&stream(
        48_000,
        2,
        AudioSampleFormat::Float32,
    ));
    let actual = ProductionOutputRouteFormatDescriptor::from_stream(&stream(
        44_100,
        2,
        AudioSampleFormat::Signed16,
    ));
    let failure = ProductionOutputRouteFormatMismatch::new(expected, actual);
    let route_failure = ProductionOutputRouteFailure::from(failure);

    assert!(failure.is_mismatch());
    assert_eq!(failure.expected().sample_rate_hz(), 48_000);
    assert_eq!(failure.actual().sample_rate_hz(), 44_100);
    assert_eq!(failure.expected().channels(), 2);
    assert_eq!(
        failure.actual().sample_format(),
        ProductionOutputRouteStreamFormat::Signed16
    );
    assert_eq!(
        route_failure.class(),
        ProductionOutputRouteFailureClass::FormatMismatch
    );
}

#[test]
fn production_output_route_failure_classes_are_distinct_contract_language() {
    let classes = [
        ProductionOutputRouteFailureClass::Backpressure,
        ProductionOutputRouteFailureClass::Underrun,
        ProductionOutputRouteFailureClass::SinkFailure,
        ProductionOutputRouteFailureClass::RouteClosed,
        ProductionOutputRouteFailureClass::FormatMismatch,
    ];
    let unique_classes = classes.into_iter().collect::<HashSet<_>>();

    assert_eq!(unique_classes.len(), 5);
}
