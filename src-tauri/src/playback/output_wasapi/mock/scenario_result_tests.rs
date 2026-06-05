use super::result::OutputThreadMockStepResult;
use super::scenario_result::OutputThreadMockScenarioResult;
use super::sequence::OutputThreadMockSequence;
use super::super::output_thread_core::plan_projection::OutputThreadStatsProjection;
use super::super::output_thread_core::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

fn audio_step(frames: u32) -> OutputThreadMockStepResult {
    OutputThreadMockStepResult {
        plan: OutputThreadRenderPlan {
            action: OutputThreadRenderAction::RenderAudio,
            frames_to_read: frames,
            silence_frames: 0,
            should_sleep: false,
            should_exit: false,
        },
        projection: OutputThreadStatsProjection {
            consumed_frames_delta: frames as u64,
            rendered_frames_delta: frames as u64,
            ..Default::default()
        },
        validation_error: None,
    }
}

fn sleep_step() -> OutputThreadMockStepResult {
    OutputThreadMockStepResult {
        plan: OutputThreadRenderPlan {
            action: OutputThreadRenderAction::Sleep,
            frames_to_read: 0,
            silence_frames: 0,
            should_sleep: true,
            should_exit: false,
        },
        projection: OutputThreadStatsProjection {
            should_sleep: true,
            ..Default::default()
        },
        validation_error: None,
    }
}

fn exit_step() -> OutputThreadMockStepResult {
    OutputThreadMockStepResult {
        plan: OutputThreadRenderPlan {
            action: OutputThreadRenderAction::Exit,
            frames_to_read: 0,
            silence_frames: 0,
            should_sleep: false,
            should_exit: true,
        },
        projection: OutputThreadStatsProjection {
            should_exit: true,
            ..Default::default()
        },
        validation_error: None,
    }
}

#[test]
fn result_new_preserves_name() {
    let seq = OutputThreadMockSequence::default().push(sleep_step());
    let result = OutputThreadMockScenarioResult::new("my_scenario", seq);
    assert_eq!(result.name, "my_scenario");
}

#[test]
fn result_new_builds_summary() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(100))
        .push(sleep_step());
    let result = OutputThreadMockScenarioResult::new("test", seq);
    assert_eq!(result.steps(), 2);
    assert_eq!(result.rendered_frames(), 100);
}

#[test]
fn valid_result_reports_valid() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(50))
        .push(sleep_step());
    let result = OutputThreadMockScenarioResult::new("valid", seq);
    assert!(result.is_valid());
}

#[test]
fn result_accessors_return_summary_values() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(200))
        .push(exit_step());
    let result = OutputThreadMockScenarioResult::new("accessor", seq);
    assert_eq!(result.steps(), 2);
    assert_eq!(result.rendered_frames(), 200);
    assert_eq!(result.silence_frames(), 0);
}

#[test]
fn result_reports_ended_and_slept() {
    let exit_seq = OutputThreadMockSequence::default().push(exit_step());
    let exit_result = OutputThreadMockScenarioResult::new("exit", exit_seq);
    assert!(exit_result.ended());
    assert!(!exit_result.slept());

    let sleep_seq = OutputThreadMockSequence::default().push(sleep_step());
    let sleep_result = OutputThreadMockScenarioResult::new("sleep", sleep_seq);
    assert!(!sleep_result.ended());
    assert!(sleep_result.slept());
}
