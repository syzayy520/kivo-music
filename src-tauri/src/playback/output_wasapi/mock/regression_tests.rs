use super::super::output_thread_control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::regression::*;
use super::renderer::OutputThreadMockRenderer;
use super::scenario::OutputThreadMockScenario;
use super::super::output_thread_state::OutputThreadState;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

#[test]
fn normal_audio_regression_succeeds() {
    let scenario = OutputThreadMockScenario::new(
        "normal_audio",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    );
    let reg = run_mock_regression(scenario);
    assert!(reg.is_success(), "failed: {:?}", reg.golden_error);
}

#[test]
fn shutdown_regression_succeeds() {
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let scenario = OutputThreadMockScenario::new(
        "shutdown_requested",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        control,
        2,
    );
    let reg = run_mock_regression(scenario);
    assert!(reg.is_success(), "failed: {:?}", reg.golden_error);
}

#[test]
fn no_capacity_regression_succeeds() {
    let scenario = OutputThreadMockScenario::new(
        "no_capacity",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::full(),
        running_control(),
        3,
    );
    let reg = run_mock_regression(scenario);
    assert!(reg.is_success(), "failed: {:?}", reg.golden_error);
}

#[test]
fn regression_result_exposes_name() {
    let scenario = OutputThreadMockScenario::new(
        "test_name",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    );
    let reg = run_mock_regression(scenario);
    assert_eq!(reg.name(), "test_name");
}

#[test]
fn regression_failure_reports_error() {
    // Build scenario with name that has no expectation
    let scenario = OutputThreadMockScenario::new(
        "nonexistent",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    );
    let reg = run_mock_regression(scenario);
    assert!(!reg.is_success());
    assert!(reg.golden_error.is_some());
}
