use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::authority::ProductionOutputRouteConfigAuthority;
use super::super::config::capacity::ProductionOutputRouteCapacity;
use super::super::config::format::ProductionOutputRouteExpectedFormat;
use super::super::config::input_acceptance::ProductionOutputRouteInputAcceptancePolicy;
use super::super::failure::ProductionOutputRouteFailureClass;
use super::super::input::ProductionOutputRouteFrameInput;
use super::super::input_admission::{
    ProductionOutputRouteInputAdmission, ProductionOutputRouteInputAdmissionResult,
};
use super::super::lifecycle::{
    ProductionOutputRouteLifecycleInputGate, ProductionOutputRouteLifecycleState,
};

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

fn accepting_policy() -> ProductionOutputRouteInputAcceptancePolicy {
    let capacity = ProductionOutputRouteCapacity::new(10).expect("capacity must be nonzero");
    let expected_format = ProductionOutputRouteExpectedFormat::from_stream(&matching_stream());
    ProductionOutputRouteInputAcceptancePolicy::new(capacity, expected_format)
}

fn rejecting_policy() -> ProductionOutputRouteInputAcceptancePolicy {
    // Small capacity that will reject when pending_frames is high
    let capacity = ProductionOutputRouteCapacity::new(1).expect("capacity must be nonzero");
    let expected_format = ProductionOutputRouteExpectedFormat::from_stream(&matching_stream());
    ProductionOutputRouteInputAcceptancePolicy::new(capacity, expected_format)
}

fn format_mismatch_policy() -> ProductionOutputRouteInputAcceptancePolicy {
    // Policy expecting different format than matching_stream
    let capacity = ProductionOutputRouteCapacity::new(10).expect("capacity must be nonzero");
    let wrong_stream = AudioStreamInfo {
        sample_rate_hz: 44_100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };
    let expected_format = ProductionOutputRouteExpectedFormat::from_stream(&wrong_stream);
    ProductionOutputRouteInputAcceptancePolicy::new(capacity, expected_format)
}

#[test]
fn input_admission_rejects_not_ready_state() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Rejected(failure) => {
            assert_eq!(
                failure.class(),
                ProductionOutputRouteFailureClass::RouteClosed
            );
        }
        other => panic!("expected Rejected, got {:?}", other),
    }
}

#[test]
fn input_admission_rejects_closed_state() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::Closed,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Rejected(failure) => {
            assert_eq!(
                failure.class(),
                ProductionOutputRouteFailureClass::RouteClosed
            );
        }
        other => panic!("expected Rejected, got {:?}", other),
    }
}

#[test]
fn input_admission_calls_config_authority_when_lifecycle_accepts() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    // Use rejecting policy with high pending_frames
    let config_authority = ProductionOutputRouteConfigAuthority::new(rejecting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        input,
        1, // pending_frames at capacity
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Rejected(failure) => {
            assert_eq!(
                failure.class(),
                ProductionOutputRouteFailureClass::Backpressure
            );
        }
        other => panic!("expected Rejected(Backpressure), got {:?}", other),
    }
}

#[test]
fn input_admission_returns_allowed_when_both_accept() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Allowed(allowed_input) => {
            assert_eq!(allowed_input.sample_count(), input.sample_count());
            assert_eq!(allowed_input.position_ms(), input.position_ms());
            assert_eq!(allowed_input.stream().sample_rate_hz, 48_000);
            assert_eq!(allowed_input.stream().channels, 2);
            assert!(matches!(
                allowed_input.stream().sample_format,
                AudioSampleFormat::Float32
            ));
        }
        other => panic!("expected Allowed, got {:?}", other),
    }
}

#[test]
fn input_admission_result_is_two_variant_enum() {
    // Exhaustive match without wildcard.
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Allowed(_) => {}
        ProductionOutputRouteInputAdmissionResult::Rejected(_) => {}
    }

    // Debug representation exists.
    let debug = format!("{:?}", result);
    assert!(!debug.is_empty());
}

#[test]
fn admit_input_uses_caller_provided_parameters() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    // Pass all parameters explicitly.
    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Allowed(allowed_input) => {
            assert_eq!(allowed_input.sample_count(), input.sample_count());
            assert_eq!(allowed_input.position_ms(), input.position_ms());
        }
        other => panic!("expected Allowed, got {:?}", other),
    }
}

#[test]
fn admit_input_returns_result_only() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let config_authority = ProductionOutputRouteConfigAuthority::new(accepting_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    // Assign to explicitly typed variable.
    let result: ProductionOutputRouteInputAdmissionResult<'_> = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::AcceptingInput,
        input,
        0,
    );

    // Exhaustive match without wildcard.
    match result {
        ProductionOutputRouteInputAdmissionResult::Allowed(_) => {}
        ProductionOutputRouteInputAdmissionResult::Rejected(_) => {}
    }

    // API returns ProductionOutputRouteInputAdmissionResult only — no Result/bool/failure wrapper.
}

#[test]
fn short_circuit_proof_lifecycle_reject_before_config() {
    let admission = ProductionOutputRouteInputAdmission;
    let lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    // Use format_mismatch_policy that would reject with FormatMismatch if reached.
    let config_authority = ProductionOutputRouteConfigAuthority::new(format_mismatch_policy());
    let frame = frame_with_stream(matching_stream());
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    // NotReadyForInput state causes lifecycle gate to reject with RouteClosed.
    // If config authority were called, it would reject with FormatMismatch.
    // But we should see RouteClosed because lifecycle gate short-circuits.
    let result = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        input,
        0,
    );

    match result {
        ProductionOutputRouteInputAdmissionResult::Rejected(failure) => {
            // This must be RouteClosed, not FormatMismatch.
            assert_eq!(
                failure.class(),
                ProductionOutputRouteFailureClass::RouteClosed,
                "lifecycle rejection must short-circuit before config authority"
            );
        }
        other => panic!("expected Rejected(RouteClosed), got {:?}", other),
    }

    // Same test for Closed state.
    let result2 = admission.admit_input(
        &lifecycle_gate,
        &config_authority,
        ProductionOutputRouteLifecycleState::Closed,
        input,
        0,
    );

    match result2 {
        ProductionOutputRouteInputAdmissionResult::Rejected(failure) => {
            assert_eq!(
                failure.class(),
                ProductionOutputRouteFailureClass::RouteClosed,
                "lifecycle rejection must short-circuit before config authority"
            );
        }
        other => panic!("expected Rejected(RouteClosed), got {:?}", other),
    }
}
