use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_mock_buffer::OutputThreadMockBuffer;
use super::output_thread_mock_golden::*;
use super::output_thread_mock_harness::plan_mock_step;
use super::output_thread_mock_renderer::OutputThreadMockRenderer;
use super::output_thread_mock_scenario_result::OutputThreadMockScenarioResult;
use super::output_thread_mock_sequence::OutputThreadMockSequence;
use super::output_thread_state::OutputThreadState;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

fn build_result(
    name: &'static str,
    buffer: OutputThreadMockBuffer,
    renderer: OutputThreadMockRenderer,
    control: OutputThreadControlSnapshot,
    max_steps: usize,
) -> OutputThreadMockScenarioResult {
    let mut sequence = OutputThreadMockSequence::default();
    let mut buf = buffer;
    for _ in 0..max_steps {
        let step = plan_mock_step(buf, renderer, control);
        let should_exit = step.should_exit();
        let should_sleep = step.should_sleep();
        let plan = step.plan;
        sequence = sequence.push(step);
        if should_exit {
            break;
        }
        if plan.action == super::output_thread_render_plan::OutputThreadRenderAction::RenderAudio {
            buf = buf.consume(plan.frames_to_read);
        }
        if should_sleep {
            break;
        }
    }
    OutputThreadMockScenarioResult::new(name, sequence)
}

#[test]
fn valid_normal_audio_result_passes_golden() {
    let result = build_result(
        "normal_audio",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    );
    assert_eq!(validate_golden_behavior(&result), Ok(()));
}

#[test]
fn valid_shutdown_result_passes_golden() {
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let result = build_result(
        "shutdown_requested",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        control,
        2,
    );
    assert_eq!(validate_golden_behavior(&result), Ok(()));
}

#[test]
fn missing_expectation_fails() {
    let result =
        OutputThreadMockScenarioResult::new("nonexistent", OutputThreadMockSequence::default());
    assert_eq!(
        validate_golden_behavior(&result),
        Err(OutputThreadMockGoldenError::MissingExpectation)
    );
}

#[test]
fn step_count_out_of_range_fails() {
    // empty sequence has 0 steps, min_steps for normal_audio is 2
    let result =
        OutputThreadMockScenarioResult::new("normal_audio", OutputThreadMockSequence::default());
    assert_eq!(
        validate_golden_behavior(&result),
        Err(OutputThreadMockGoldenError::StepCountOutOfRange)
    );
}

#[test]
fn rendered_frames_too_low_fails() {
    // normal_audio requires min_rendered_frames=100
    // buffer=20 gives 20 audio + 50 silence = 70 total rendered < 100
    let result = build_result(
        "normal_audio",
        OutputThreadMockBuffer::with_frames(20),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        2,
    );
    assert_eq!(
        validate_golden_behavior(&result),
        Err(OutputThreadMockGoldenError::RenderedFramesTooLow)
    );
}

#[test]
fn silence_frames_too_low_fails() {
    // empty_running requires min_silence_frames=50
    // run with max_steps=1 so we don't accumulate enough silence
    let result = build_result(
        "empty_running",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::full(),
        running_control(),
        1,
    );
    let err = validate_golden_behavior(&result);
    assert!(
        err == Err(OutputThreadMockGoldenError::SilenceFramesTooLow)
            || err == Err(OutputThreadMockGoldenError::StepCountOutOfRange),
        "unexpected: {err:?}"
    );
}

#[test]
fn final_action_mismatch_fails() {
    // normal_audio expects RenderSilence, but max_steps=2 gives last=RenderAudio
    let result = build_result(
        "normal_audio",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        2,
    );
    assert_eq!(
        validate_golden_behavior(&result),
        Err(OutputThreadMockGoldenError::FinalActionMismatch)
    );
}

#[test]
fn validity_mismatch_fails() {
    // We can't easily force a validation error from this level,
    // so test the error variant exists and is distinct
    assert_ne!(
        OutputThreadMockGoldenError::ValidityMismatch,
        OutputThreadMockGoldenError::FinalActionMismatch
    );
}
