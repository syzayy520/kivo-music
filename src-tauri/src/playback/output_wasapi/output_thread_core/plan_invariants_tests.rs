use super::plan_invariants::validate_render_plan_shape;
use super::plan_validation::OutputThreadPlanValidationError;
use super::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

#[test]
fn valid_sleep_plan_passes() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Sleep,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: true,
        should_exit: false,
    };
    assert!(validate_render_plan_shape(plan).is_ok());
}

#[test]
fn valid_audio_plan_passes() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 128,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    };
    assert!(validate_render_plan_shape(plan).is_ok());
}

#[test]
fn valid_silence_plan_passes() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderSilence,
        frames_to_read: 0,
        silence_frames: 64,
        should_sleep: false,
        should_exit: false,
    };
    assert!(validate_render_plan_shape(plan).is_ok());
}

#[test]
fn valid_exit_plan_passes() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: true,
    };
    assert!(validate_render_plan_shape(plan).is_ok());
}

#[test]
fn audio_plan_without_frames_fails() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    };
    assert_eq!(
        validate_render_plan_shape(plan),
        Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames)
    );
}

#[test]
fn audio_plan_with_silence_frames_fails() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 64,
        silence_frames: 32,
        should_sleep: false,
        should_exit: false,
    };
    assert_eq!(
        validate_render_plan_shape(plan),
        Err(OutputThreadPlanValidationError::AudioPlanWithSilenceFrames)
    );
}

#[test]
fn silence_plan_without_frames_fails() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderSilence,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    };
    assert_eq!(
        validate_render_plan_shape(plan),
        Err(OutputThreadPlanValidationError::SilencePlanWithoutFrames)
    );
}

#[test]
fn sleep_plan_with_work_fails() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Sleep,
        frames_to_read: 10,
        silence_frames: 0,
        should_sleep: true,
        should_exit: false,
    };
    assert_eq!(
        validate_render_plan_shape(plan),
        Err(OutputThreadPlanValidationError::SleepPlanHasWork)
    );
}

#[test]
fn exit_plan_with_work_fails() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        frames_to_read: 0,
        silence_frames: 5,
        should_sleep: false,
        should_exit: true,
    };
    assert_eq!(
        validate_render_plan_shape(plan),
        Err(OutputThreadPlanValidationError::ExitPlanHasWork)
    );
}
