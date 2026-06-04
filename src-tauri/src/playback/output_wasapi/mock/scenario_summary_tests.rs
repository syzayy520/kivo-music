use super::output_thread_mock_result::OutputThreadMockStepResult;
use super::output_thread_mock_scenario_summary::OutputThreadMockScenarioSummary;
use super::output_thread_mock_sequence::OutputThreadMockSequence;
use super::output_thread_plan_projection::OutputThreadStatsProjection;
use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

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

fn silence_step(frames: u32) -> OutputThreadMockStepResult {
    OutputThreadMockStepResult {
        plan: OutputThreadRenderPlan {
            action: OutputThreadRenderAction::RenderSilence,
            frames_to_read: 0,
            silence_frames: frames,
            should_sleep: false,
            should_exit: false,
        },
        projection: OutputThreadStatsProjection {
            rendered_frames_delta: frames as u64,
            silence_filled_frames_delta: frames as u64,
            ..Default::default()
        },
        validation_error: None,
    }
}

#[test]
fn summary_from_empty_sequence_is_zeroed() {
    let seq = OutputThreadMockSequence::default();
    let summary = OutputThreadMockScenarioSummary::from_sequence(&seq);
    assert_eq!(summary.steps, 0);
    assert_eq!(summary.rendered_frames, 0);
    assert_eq!(summary.silence_frames, 0);
    assert!(!summary.has_errors);
    assert!(!summary.ended);
    assert!(!summary.slept);
}

#[test]
fn summary_detects_sleeping_final_step() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(50))
        .push(sleep_step());
    let summary = OutputThreadMockScenarioSummary::from_sequence(&seq);
    assert!(summary.slept);
    assert!(!summary.ended);
}

#[test]
fn summary_detects_exit_final_step() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(50))
        .push(exit_step());
    let summary = OutputThreadMockScenarioSummary::from_sequence(&seq);
    assert!(summary.ended);
    assert!(!summary.slept);
}

#[test]
fn summary_counts_rendered_and_silence_frames() {
    let seq = OutputThreadMockSequence::default()
        .push(audio_step(100))
        .push(silence_step(50))
        .push(sleep_step());
    let summary = OutputThreadMockScenarioSummary::from_sequence(&seq);
    assert_eq!(summary.rendered_frames, 150);
    assert_eq!(summary.silence_frames, 50);
}

#[test]
fn summary_detects_validation_errors() {
    let mut step = sleep_step();
    step.validation_error = Some(
        super::output_thread_plan_validation::OutputThreadPlanValidationError::SleepPlanHasWork,
    );
    let seq = OutputThreadMockSequence::default().push(step);
    let summary = OutputThreadMockScenarioSummary::from_sequence(&seq);
    assert!(summary.has_errors);
}
