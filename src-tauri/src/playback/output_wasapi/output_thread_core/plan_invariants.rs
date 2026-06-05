use super::plan_validation::{
    OutputThreadPlanValidationError, OutputThreadPlanValidationResult,
};
use super::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

/// Validate that a render plan's internal fields are self-consistent.
///
/// Pure shape check — no control snapshot, no buffer state.
#[allow(dead_code)]
pub(crate) fn validate_render_plan_shape(
    plan: OutputThreadRenderPlan,
) -> OutputThreadPlanValidationResult {
    match plan.action {
        OutputThreadRenderAction::Sleep => {
            if plan.frames_to_read != 0 || plan.silence_frames != 0 {
                return Err(OutputThreadPlanValidationError::SleepPlanHasWork);
            }
            if !plan.should_sleep || plan.should_exit {
                return Err(OutputThreadPlanValidationError::SleepPlanHasWork);
            }
        }
        OutputThreadRenderAction::RenderAudio => {
            if plan.frames_to_read == 0 {
                return Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames);
            }
            if plan.silence_frames != 0 {
                return Err(OutputThreadPlanValidationError::AudioPlanWithSilenceFrames);
            }
            if plan.should_sleep || plan.should_exit {
                return Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames);
            }
        }
        OutputThreadRenderAction::RenderSilence => {
            if plan.silence_frames == 0 {
                return Err(OutputThreadPlanValidationError::SilencePlanWithoutFrames);
            }
            if plan.frames_to_read != 0 {
                return Err(OutputThreadPlanValidationError::SilencePlanWithAudioFrames);
            }
            if plan.should_sleep || plan.should_exit {
                return Err(OutputThreadPlanValidationError::SilencePlanWithoutFrames);
            }
        }
        OutputThreadRenderAction::Exit => {
            if plan.frames_to_read != 0 || plan.silence_frames != 0 {
                return Err(OutputThreadPlanValidationError::ExitPlanHasWork);
            }
            if plan.should_sleep || !plan.should_exit {
                return Err(OutputThreadPlanValidationError::ExitPlanHasWork);
            }
        }
    }
    Ok(())
}
