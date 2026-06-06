use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

use super::super::{
    config::{
        capacity::ProductionOutputRouteCapacity, format::ProductionOutputRouteExpectedFormat,
    },
    ProductionOutputRouteFormatDescriptor,
};

fn stream(sample_rate_hz: u32, channels: u16, sample_format: AudioSampleFormat) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format,
    }
}

#[test]
fn production_output_route_config_accepts_positive_capacity() {
    let capacity = ProductionOutputRouteCapacity::new(4).expect("positive capacity");

    assert_eq!(capacity.capacity_frames(), 4);
    assert_eq!(capacity.accept(3, 1), Ok(()));
}

#[test]
fn production_output_route_config_rejects_zero_capacity() {
    assert_eq!(ProductionOutputRouteCapacity::new(0), None);
}

#[test]
fn production_output_route_config_rejects_projected_capacity_overflow() {
    let capacity = ProductionOutputRouteCapacity::new(4).expect("positive capacity");
    let failure = capacity.accept(3, 2).expect_err("capacity exceeded");

    assert_eq!(failure.pending_frames(), 5);
    assert_eq!(failure.capacity_frames(), 4);
    assert!(failure.is_capacity_reached());
}

#[test]
fn production_output_route_config_expected_format_accepts_matching_descriptor() {
    let stream = stream(48_000, 2, AudioSampleFormat::Float32);
    let expected = ProductionOutputRouteExpectedFormat::from_stream(&stream);
    let actual = ProductionOutputRouteFormatDescriptor::from_stream(&stream);

    assert_eq!(expected.descriptor(), actual);
    assert_eq!(expected.accept(actual), Ok(()));
}

#[test]
fn production_output_route_config_expected_format_rejects_mismatch() {
    let expected_stream = stream(48_000, 2, AudioSampleFormat::Float32);
    let actual_stream = stream(44_100, 2, AudioSampleFormat::Float32);
    let expected = ProductionOutputRouteExpectedFormat::from_stream(&expected_stream);
    let actual = ProductionOutputRouteFormatDescriptor::from_stream(&actual_stream);
    let failure = expected.accept(actual).expect_err("format mismatch");

    assert_eq!(failure.expected(), expected.descriptor());
    assert_eq!(failure.actual(), actual);
    assert!(failure.is_mismatch());
}
