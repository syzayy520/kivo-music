use super::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

/// Projected stats delta from a single render plan step.
///
/// Pure projection — does not modify real stats or read any buffer.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadStatsProjection {
    /// Frames consumed from the buffer.
    pub consumed_frames_delta: u64,
    /// Frames rendered to the output device.
    pub rendered_frames_delta: u64,
    /// Silence frames filled.
    pub silence_filled_frames_delta: u64,
    /// Frames dropped.
    pub dropped_frames_delta: u64,
    /// Whether the consumer should sleep.
    pub should_sleep: bool,
    /// Whether the consumer should exit.
    pub should_exit: bool,
}

/// Project stats delta from a render plan.
///
/// Pure function — no buffer reads, no device calls, no state mutation.
#[allow(dead_code)]
pub(crate) fn project_stats_from_plan(plan: OutputThreadRenderPlan) -> OutputThreadStatsProjection {
    match plan.action {
        OutputThreadRenderAction::RenderAudio => OutputThreadStatsProjection {
            consumed_frames_delta: plan.frames_to_read as u64,
            rendered_frames_delta: plan.frames_to_read as u64,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: false,
        },
        OutputThreadRenderAction::RenderSilence => OutputThreadStatsProjection {
            consumed_frames_delta: 0,
            rendered_frames_delta: plan.silence_frames as u64,
            silence_filled_frames_delta: plan.silence_frames as u64,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: false,
        },
        OutputThreadRenderAction::Sleep => OutputThreadStatsProjection {
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: true,
            should_exit: false,
        },
        OutputThreadRenderAction::Exit => OutputThreadStatsProjection {
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: true,
        },
    }
}
