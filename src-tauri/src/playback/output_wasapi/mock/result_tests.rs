use super::output_thread_mock_result::OutputThreadMockStepResult;
use super::output_thread_plan_projection::OutputThreadStatsProjection;
use super::output_thread_plan_validation::OutputThreadPlanValidationError;
use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

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
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: false,
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
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: true,
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
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: true,
            should_exit: false,
        },
        validation_error: None,
    }
}

fn invalid_step() -> OutputThreadMockStepResult {
    OutputThreadMockStepResult {
        plan: OutputThreadRenderPlan {
            action: OutputThreadRenderAction::RenderAudio,
            frames_to_read: 0,
            silence_frames: 0,
            should_sleep: false,
            should_exit: false,
        },
        projection: OutputThreadStatsProjection::default(),
        validation_error: Some(OutputThreadPlanValidationError::AudioPlanWithoutFrames),
    }
}

#[test]
fn valid_result_reports_valid() {
    let result = audio_step(128);
    assert!(result.is_valid());
}

#[test]
fn invalid_result_reports_error() {
    let result = invalid_step();
    assert!(!result.is_valid());
    assert_eq!(
        result.validation_error,
        Some(OutputThreadPlanValidationError::AudioPlanWithoutFrames)
    );
}

#[test]
fn result_for_exit_plan_should_exit() {
    let result = exit_step();
    assert!(result.should_exit());
    assert!(!result.should_sleep());
}

#[test]
fn result_for_sleep_plan_should_sleep() {
    let result = sleep_step();
    assert!(result.should_sleep());
    assert!(!result.should_exit());
}
