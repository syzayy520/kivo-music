use super::super::output_thread_core::control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::renderer::OutputThreadMockRenderer;
use super::scenario::OutputThreadMockScenario;
use super::scenario_runner::run_mock_scenario;
use super::super::output_thread_core::state::OutputThreadState;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

#[test]
fn runner_preserves_scenario_name() {
    let scenario = OutputThreadMockScenario::new(
        "preserved_name",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    );
    let result = run_mock_scenario(scenario);
    assert_eq!(result.name, "preserved_name");
}

#[test]
fn runner_executes_normal_audio_scenario() {
    let scenario = OutputThreadMockScenario::new(
        "normal",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    );
    let result = run_mock_scenario(scenario);
    assert!(result.rendered_frames() > 0);
    assert!(result.is_valid());
}

#[test]
fn runner_executes_shutdown_scenario_as_exit() {
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let scenario = OutputThreadMockScenario::new(
        "shutdown",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        control,
        2,
    );
    let result = run_mock_scenario(scenario);
    assert!(result.ended());
    assert_eq!(result.steps(), 1);
}

#[test]
fn runner_executes_no_capacity_scenario_as_sleep() {
    let scenario = OutputThreadMockScenario::new(
        "no_cap",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    );
    let result = run_mock_scenario(scenario);
    assert!(result.slept());
}

#[test]
fn runner_does_not_modify_factory_inputs() {
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let renderer = OutputThreadMockRenderer::with_free_frames(50);
    let control = running_control();
    let scenario = OutputThreadMockScenario::new("immutable", buffer, renderer, control, 2);
    let _result = run_mock_scenario(scenario);
    // Original values should be unchanged (passed by value, not mutated)
    assert_eq!(buffer.snapshot().available_frames, 100);
    assert_eq!(renderer.free_frames(), 50);
}
