use std::fmt::Debug;

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::{
    ProductionOutputRouteBackpressure, ProductionOutputRouteFailure,
    ProductionOutputRouteFailureClass, ProductionOutputRouteFormatDescriptor,
    ProductionOutputRouteFormatMismatch, ProductionOutputRouteFrameInput,
    ProductionOutputRouteRouteClosed, ProductionOutputRouteRouteClosedReason,
    ProductionOutputRouteSinkFailure, ProductionOutputRouteStreamFormat,
    ProductionOutputRouteUnderrun,
};

fn assert_copy_debug_contract<T: Copy + Debug>() {}

fn assert_value_failure_contract<T>()
where
    T: Into<ProductionOutputRouteFailure> + Clone + Debug + PartialEq,
{
}

fn stream(sample_format: AudioSampleFormat) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format,
    }
}

fn frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream(AudioSampleFormat::Float32),
        position_ms: 960,
        samples: vec![0.0, 0.125, -0.125, 0.0],
    }
}

#[test]
fn production_output_route_boundary_input_is_borrowed_metadata_contract() {
    assert_copy_debug_contract::<ProductionOutputRouteFrameInput<'_>>();

    let frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);
    let copied_input = input;

    assert_eq!(copied_input.position_ms(), frame.position_ms);
    assert_eq!(copied_input.stream().channels, frame.stream.channels);
    assert_eq!(copied_input.sample_count(), frame.samples.len());
}

#[test]
fn production_output_route_boundary_failures_are_plain_value_contracts() {
    assert_value_failure_contract::<ProductionOutputRouteBackpressure>();
    assert_value_failure_contract::<ProductionOutputRouteUnderrun>();
    assert_value_failure_contract::<ProductionOutputRouteSinkFailure>();
    assert_value_failure_contract::<ProductionOutputRouteRouteClosed>();
    assert_value_failure_contract::<ProductionOutputRouteFormatMismatch>();

    assert_copy_debug_contract::<ProductionOutputRouteFailureClass>();
    assert_copy_debug_contract::<ProductionOutputRouteRouteClosedReason>();
    assert_copy_debug_contract::<ProductionOutputRouteStreamFormat>();
    assert_copy_debug_contract::<ProductionOutputRouteFormatDescriptor>();
}

#[test]
fn production_output_route_boundary_classifies_without_runtime_attachment() {
    let expected =
        ProductionOutputRouteFormatDescriptor::from_stream(&stream(AudioSampleFormat::Float32));
    let actual =
        ProductionOutputRouteFormatDescriptor::from_stream(&stream(AudioSampleFormat::Signed16));

    let failures = [
        ProductionOutputRouteFailure::from(ProductionOutputRouteBackpressure::new(4, 4)),
        ProductionOutputRouteFailure::from(ProductionOutputRouteUnderrun::new(512, 256)),
        ProductionOutputRouteFailure::from(ProductionOutputRouteSinkFailure::new(
            "submit_frame",
            "sink rejected frame",
        )),
        ProductionOutputRouteFailure::from(ProductionOutputRouteRouteClosed::new(
            ProductionOutputRouteRouteClosedReason::NotOpened,
        )),
        ProductionOutputRouteFailure::from(ProductionOutputRouteFormatMismatch::new(
            expected, actual,
        )),
    ];

    let classes = failures.map(|failure| failure.class());

    assert_eq!(
        classes,
        [
            ProductionOutputRouteFailureClass::Backpressure,
            ProductionOutputRouteFailureClass::Underrun,
            ProductionOutputRouteFailureClass::SinkFailure,
            ProductionOutputRouteFailureClass::RouteClosed,
            ProductionOutputRouteFailureClass::FormatMismatch,
        ]
    );
}
