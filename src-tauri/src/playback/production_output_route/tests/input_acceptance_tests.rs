use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::{
    config::{
        capacity::ProductionOutputRouteCapacity, format::ProductionOutputRouteExpectedFormat,
        input_acceptance::ProductionOutputRouteInputAcceptancePolicy,
    },
    ProductionOutputRouteFailure, ProductionOutputRouteFailureClass,
    ProductionOutputRouteFrameInput,
};

fn stream(sample_rate_hz: u32, channels: u16, sample_format: AudioSampleFormat) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format,
    }
}

fn frame(stream: AudioStreamInfo) -> AudioOutputFrame {
    AudioOutputFrame {
        stream,
        position_ms: 240,
        samples: vec![0.0, 0.25, -0.25, 0.0],
    }
}

fn policy(
    expected_stream: &AudioStreamInfo,
    capacity_frames: usize,
) -> ProductionOutputRouteInputAcceptancePolicy {
    ProductionOutputRouteInputAcceptancePolicy::new(
        ProductionOutputRouteCapacity::new(capacity_frames).expect("positive capacity"),
        ProductionOutputRouteExpectedFormat::from_stream(expected_stream),
    )
}

fn assert_format_mismatch(
    result: Result<(), ProductionOutputRouteFailure>,
) -> ProductionOutputRouteFailure {
    let failure = result.expect_err("format mismatch");
    assert_eq!(
        failure.class(),
        ProductionOutputRouteFailureClass::FormatMismatch
    );
    failure
}

#[test]
fn production_output_route_input_acceptance_accepts_matching_frame() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(expected.clone());
    let acceptance = policy(&expected, 2);

    assert_eq!(
        acceptance.accept(ProductionOutputRouteFrameInput::from_frame(&frame), 0),
        Ok(())
    );
}

#[test]
fn production_output_route_input_acceptance_rejects_sample_rate_mismatch() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(stream(44_100, 2, AudioSampleFormat::Float32));

    assert_format_mismatch(
        policy(&expected, 2).accept(ProductionOutputRouteFrameInput::from_frame(&frame), 0),
    );
}

#[test]
fn production_output_route_input_acceptance_rejects_channel_mismatch() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(stream(48_000, 1, AudioSampleFormat::Float32));

    assert_format_mismatch(
        policy(&expected, 2).accept(ProductionOutputRouteFrameInput::from_frame(&frame), 0),
    );
}

#[test]
fn production_output_route_input_acceptance_rejects_sample_format_mismatch() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(stream(48_000, 2, AudioSampleFormat::Signed16));

    assert_format_mismatch(
        policy(&expected, 2).accept(ProductionOutputRouteFrameInput::from_frame(&frame), 0),
    );
}

#[test]
fn production_output_route_input_acceptance_rejects_reached_capacity() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(expected.clone());
    let failure = policy(&expected, 1)
        .accept(ProductionOutputRouteFrameInput::from_frame(&frame), 1)
        .expect_err("capacity reached");

    assert_eq!(
        failure.class(),
        ProductionOutputRouteFailureClass::Backpressure
    );
    match failure {
        ProductionOutputRouteFailure::Backpressure(backpressure) => {
            assert_eq!(backpressure.pending_frames(), 2);
            assert_eq!(backpressure.capacity_frames(), 1);
        }
        other => panic!("unexpected failure: {other:?}"),
    }
}

#[test]
fn production_output_route_input_acceptance_returns_unit_without_owning_samples() {
    let expected = stream(48_000, 2, AudioSampleFormat::Float32);
    let frame = frame(expected.clone());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);
    let result: Result<(), ProductionOutputRouteFailure> = policy(&expected, 1).accept(input, 0);

    assert_eq!(result, Ok(()));
    assert_eq!(input.sample_count(), frame.samples.len());
}
