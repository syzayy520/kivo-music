use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::failure::{ProductionOutputRouteFailureClass, ProductionOutputRouteRouteClosedReason};
use super::super::input::ProductionOutputRouteFrameInput;
use super::super::lifecycle::{
    ProductionOutputRouteLifecycleInputGate, ProductionOutputRouteLifecycleState,
};

fn stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream(),
        position_ms: 960,
        samples: vec![0.0, 0.125, -0.125, 0.0],
    }
}

#[test]
fn lifecycle_input_gate_accepting_input_allows_frame_input() {
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    let result = gate.gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input);

    let accepted = result.expect("AcceptingInput should allow input");
    assert_eq!(accepted.position_ms(), 960);
    assert_eq!(accepted.sample_count(), 4);
}

#[test]
fn lifecycle_input_gate_not_ready_for_input_rejects_with_route_closed_not_opened() {
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    let err = gate
        .gate_input(ProductionOutputRouteLifecycleState::NotReadyForInput, input)
        .expect_err("NotReadyForInput should reject");

    assert_eq!(err.class(), ProductionOutputRouteFailureClass::RouteClosed);
    match err {
        super::super::failure::ProductionOutputRouteFailure::RouteClosed(closed) => {
            assert_eq!(closed.reason(), ProductionOutputRouteRouteClosedReason::NotOpened);
        }
        _ => panic!("expected RouteClosed variant"),
    }
}

#[test]
fn lifecycle_input_gate_closed_rejects_with_route_closed_rejected_after_close() {
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    let err = gate
        .gate_input(ProductionOutputRouteLifecycleState::Closed, input)
        .expect_err("Closed should reject");

    assert_eq!(err.class(), ProductionOutputRouteFailureClass::RouteClosed);
    match err {
        super::super::failure::ProductionOutputRouteFailure::RouteClosed(closed) => {
            assert_eq!(closed.reason(), ProductionOutputRouteRouteClosedReason::RejectedAfterClose);
        }
        _ => panic!("expected RouteClosed variant"),
    }
}

#[test]
fn lifecycle_input_gate_does_not_check_format() {
    // Gate accepts any frame regardless of format — no format assertion inside gate.
    // The gate API takes state + input, returns Ok/Err based on state only.
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    // AcceptingInput passes through regardless of stream format.
    let result = gate.gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input);
    assert!(result.is_ok());
}

#[test]
fn lifecycle_input_gate_does_not_check_capacity_or_pending_frames() {
    // gate_input takes (state, input) only — no pending_frames parameter.
    // No Backpressure construction possible inside gate.
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    // AcceptingInput passes through without capacity check.
    let result = gate.gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input);
    assert!(result.is_ok());
}

#[test]
fn lifecycle_input_gate_does_not_consume_or_clone_samples() {
    // Accepted input remains a borrowed wrapper — no owned AudioOutputFrame return.
    // gate_input returns Ok(ProductionOutputRouteFrameInput) which wraps &AudioOutputFrame.
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    let accepted = gate
        .gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input)
        .expect("should accept");

    // The accepted input still references the same borrowed frame.
    assert_eq!(accepted.position_ms(), audio_frame.position_ms);
    assert_eq!(accepted.sample_count(), audio_frame.samples.len());
}

#[test]
fn lifecycle_input_gate_does_not_call_config_authority() {
    // API shape proof: gate_input takes (state, input) — no config authority parameter.
    // No InputAcceptancePolicy, no ConfigAuthority in the imports of input_gate.rs.
    let gate = ProductionOutputRouteLifecycleInputGate::new();
    let audio_frame = frame();
    let input = ProductionOutputRouteFrameInput::from_frame(&audio_frame);

    // Just verifying the API compiles and works without config authority.
    let result = gate.gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input);
    assert!(result.is_ok());
}

#[test]
fn lifecycle_input_gate_is_stateless() {
    // Multiple calls with different states do not retain previous result.
    let gate = ProductionOutputRouteLifecycleInputGate::new();

    // First call: Closed -> reject
    let audio_frame1 = frame();
    let input1 = ProductionOutputRouteFrameInput::from_frame(&audio_frame1);
    let err = gate
        .gate_input(ProductionOutputRouteLifecycleState::Closed, input1)
        .expect_err("Closed should reject");
    assert_eq!(err.class(), ProductionOutputRouteFailureClass::RouteClosed);

    // Second call: AcceptingInput -> accept (no retained state from previous rejection)
    let audio_frame2 = frame();
    let input2 = ProductionOutputRouteFrameInput::from_frame(&audio_frame2);
    let ok = gate
        .gate_input(ProductionOutputRouteLifecycleState::AcceptingInput, input2)
        .expect("AcceptingInput should accept after previous Closed");
    assert_eq!(ok.position_ms(), 960);

    // Third call: NotReadyForInput -> reject
    let audio_frame3 = frame();
    let input3 = ProductionOutputRouteFrameInput::from_frame(&audio_frame3);
    let err = gate
        .gate_input(ProductionOutputRouteLifecycleState::NotReadyForInput, input3)
        .expect_err("NotReadyForInput should reject");
    assert_eq!(err.class(), ProductionOutputRouteFailureClass::RouteClosed);
}
