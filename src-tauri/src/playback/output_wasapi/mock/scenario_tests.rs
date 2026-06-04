use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_mock_buffer::OutputThreadMockBuffer;
use super::output_thread_mock_renderer::OutputThreadMockRenderer;
use super::output_thread_mock_scenario::OutputThreadMockScenario;
use super::output_thread_state::OutputThreadState;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

#[test]
fn scenario_new_preserves_fields() {
    let scenario = OutputThreadMockScenario::new(
        "test",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    );
    assert_eq!(scenario.name, "test");
    assert_eq!(scenario.max_steps, 4);
}

#[test]
fn scenario_is_bounded_when_max_steps_positive() {
    let scenario = OutputThreadMockScenario::new(
        "bounded",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        10,
    );
    assert!(scenario.is_bounded());
}

#[test]
fn scenario_is_unbounded_when_max_steps_zero() {
    let scenario = OutputThreadMockScenario::new(
        "unbounded",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        0,
    );
    assert!(!scenario.is_bounded());
}

#[test]
fn scenario_has_capacity_matches_renderer() {
    let with_cap = OutputThreadMockScenario::new(
        "has_cap",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        2,
    );
    assert!(with_cap.has_capacity());

    let no_cap = OutputThreadMockScenario::new(
        "no_cap",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    );
    assert!(!no_cap.has_capacity());
}

#[test]
fn scenario_max_steps_accessor_returns_value() {
    let scenario = OutputThreadMockScenario::new(
        "accessor",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        42,
    );
    assert_eq!(scenario.max_steps(), 42);
}
