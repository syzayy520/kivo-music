use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::authority::ProductionOutputRouteConfigAuthority;
use super::super::config::capacity::ProductionOutputRouteCapacity;
use super::super::config::format::ProductionOutputRouteExpectedFormat;
use super::super::config::input_acceptance::ProductionOutputRouteInputAcceptancePolicy;
use super::super::failure::ProductionOutputRouteFailure;
use super::super::ProductionOutputRouteFrameInput;

fn matching_stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn frame_with_stream(stream: AudioStreamInfo) -> AudioOutputFrame {
    AudioOutputFrame {
        stream,
        position_ms: 0,
        samples: vec![0.0, 0.25, -0.25, 0.0],
    }
}

fn policy_with_capacity(capacity_frames: usize) -> ProductionOutputRouteInputAcceptancePolicy {
    let capacity = ProductionOutputRouteCapacity::new(capacity_frames)
        .expect("capacity must be nonzero");
    let expected_format =
        ProductionOutputRouteExpectedFormat::from_stream(&matching_stream());
    ProductionOutputRouteInputAcceptancePolicy::new(capacity, expected_format)
}

#[test]
fn config_authority_delegates_matching_input() {
    let policy = policy_with_capacity(10);
    let authority = ProductionOutputRouteConfigAuthority::new(policy);
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = authority.accept(input, 0);
    assert!(result.is_ok(), "matching input should be accepted");
}

#[test]
fn config_authority_returns_format_mismatch() {
    let policy = policy_with_capacity(10);
    let authority = ProductionOutputRouteConfigAuthority::new(policy);

    let wrong_stream = AudioStreamInfo {
        sample_rate_hz: 44_100, // different from 48_000
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };
    let frame = frame_with_stream(wrong_stream);
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let err = authority.accept(input, 0).expect_err("format mismatch expected");
    match err {
        ProductionOutputRouteFailure::FormatMismatch(_) => {}
        other => panic!(
            "expected FormatMismatch, got {:?}",
            ProductionOutputRouteFailure::class(&other)
        ),
    }
}

#[test]
fn config_authority_returns_backpressure() {
    let policy = policy_with_capacity(5);
    let authority = ProductionOutputRouteConfigAuthority::new(policy);
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    // pending_frames already at capacity
    let err = authority.accept(input, 5).expect_err("backpressure expected");
    match err {
        ProductionOutputRouteFailure::Backpressure(_) => {}
        other => panic!(
            "expected Backpressure, got {:?}",
            ProductionOutputRouteFailure::class(&other)
        ),
    }
}

#[test]
fn config_authority_pending_frames_is_explicit_not_retained() {
    let policy = policy_with_capacity(10);
    let authority = ProductionOutputRouteConfigAuthority::new(policy);
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    // First call with pending=0 should succeed
    assert!(authority.accept(input, 0).is_ok());

    // Second call with pending=9 should succeed (9 + 1 = 10 == capacity)
    assert!(authority.accept(input, 9).is_ok());

    // Third call with pending=10 should fail (10 + 1 > capacity)
    assert!(authority.accept(input, 10).is_err());
}

#[test]
fn config_authority_result_carries_no_frame_data() {
    let policy = policy_with_capacity(10);
    let authority = ProductionOutputRouteConfigAuthority::new(policy);
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = authority.accept(input, 0);
    assert!(result.is_ok());
    // Result is Result<(), ProductionOutputRouteFailure> — no sample data possible
}
