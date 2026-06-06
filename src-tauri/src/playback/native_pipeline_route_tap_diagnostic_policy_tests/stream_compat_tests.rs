use crate::playback::decoder::AudioSampleFormat;
use crate::playback::native_pipeline_route_tap_diagnostic_policy::is_same_diagnostic_route_stream;

use super::fixtures::stream;

#[test]
fn identical_stream_fields_are_compatible() {
    let left = stream(48_000, 2, AudioSampleFormat::Float32);
    let right = stream(48_000, 2, AudioSampleFormat::Float32);

    assert!(is_same_diagnostic_route_stream(&left, &right));
}

#[test]
fn different_sample_rate_is_incompatible() {
    let left = stream(48_000, 2, AudioSampleFormat::Float32);
    let right = stream(44_100, 2, AudioSampleFormat::Float32);

    assert!(!is_same_diagnostic_route_stream(&left, &right));
}

#[test]
fn different_channel_count_is_incompatible() {
    let left = stream(48_000, 2, AudioSampleFormat::Float32);
    let right = stream(48_000, 1, AudioSampleFormat::Float32);

    assert!(!is_same_diagnostic_route_stream(&left, &right));
}

#[test]
fn different_sample_format_is_incompatible() {
    let left = stream(48_000, 2, AudioSampleFormat::Float32);
    let right = stream(48_000, 2, AudioSampleFormat::Signed16);

    assert!(!is_same_diagnostic_route_stream(&left, &right));
}
