use super::buffer_snapshot::OutputThreadBufferSnapshot;
use super::control::OutputThreadControlSnapshot;
use super::plan_validation::{
    OutputThreadPlanValidationError, OutputThreadPlanValidationResult,
};
use super::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::state::OutputThreadState;

/// Validate that a render plan is legal for the given control and buffer state.
///
/// Pure validation — no buffer reads, no device calls, no state mutation.
#[allow(dead_code)]
pub(crate) fn validate_plan_for_state(
    buffer: OutputThreadBufferSnapshot,
    control: OutputThreadControlSnapshot,
    plan: OutputThreadRenderPlan,
) -> OutputThreadPlanValidationResult {
    // Rule 1: shutdown requires Exit.
    if control.shutdown_requested && plan.action != OutputThreadRenderAction::Exit {
        return Err(OutputThreadPlanValidationError::ShutdownPlanMustExit);
    }

    // Rule 2: closed + empty buffer requires Exit.
    if buffer.is_closed
        && buffer.available_frames == 0
        && plan.action != OutputThreadRenderAction::Exit
    {
        return Err(OutputThreadPlanValidationError::ClosedExhaustedPlanMustExit);
    }

    // Rule 3: non-running (not shutdown/closed-exhausted) requires Sleep.
    if control.state != OutputThreadState::Running
        && !control.shutdown_requested
        && !(buffer.is_closed && buffer.available_frames == 0)
        && plan.action != OutputThreadRenderAction::Sleep
    {
        return Err(OutputThreadPlanValidationError::NonRunningPlanMustSleep);
    }

    // Rule 4: paused rejects RenderAudio.
    if control.paused && plan.action == OutputThreadRenderAction::RenderAudio {
        return Err(OutputThreadPlanValidationError::PausedPlanMustNotRenderAudio);
    }

    // Rule 5: flush rejects RenderAudio.
    if control.flush_requested && plan.action == OutputThreadRenderAction::RenderAudio {
        return Err(OutputThreadPlanValidationError::FlushPlanMustNotRenderAudio);
    }

    // Rule 6: empty running buffer (not closed-exhausted, not paused/flush, not shutdown)
    // rejects RenderAudio.
    if buffer.available_frames == 0
        && !(buffer.is_closed)
        && control.state == OutputThreadState::Running
        && !control.paused
        && !control.flush_requested
        && !control.shutdown_requested
        && plan.action == OutputThreadRenderAction::RenderAudio
    {
        return Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames);
    }

    // Rule 7: audio plan cannot exceed available frames.
    if plan.action == OutputThreadRenderAction::RenderAudio
        && plan.frames_to_read > buffer.available_frames
    {
        return Err(OutputThreadPlanValidationError::AudioPlanExceedsAvailable);
    }

    // Rule 8: audio plan must have frames_to_read > 0.
    if plan.action == OutputThreadRenderAction::RenderAudio && plan.frames_to_read == 0 {
        return Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames);
    }

    // Rule 9: silence plan must have silence_frames > 0.
    if plan.action == OutputThreadRenderAction::RenderSilence && plan.silence_frames == 0 {
        return Err(OutputThreadPlanValidationError::SilencePlanWithoutFrames);
    }

    Ok(())
}
