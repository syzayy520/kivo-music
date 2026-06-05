use super::super::output_thread_core::render_plan::OutputThreadRenderAction;

/// Expectation for a mock scenario outcome.
///
/// Pure data — describes expected bounds, does not run scenarios.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadMockExpectation {
    /// Expected final action of the sequence.
    pub expected_final_action: OutputThreadRenderAction,
    /// Minimum number of steps.
    pub min_steps: usize,
    /// Maximum number of steps.
    pub max_steps: usize,
    /// Minimum rendered audio frames.
    pub min_rendered_frames: u64,
    /// Minimum silence frames filled.
    pub min_silence_frames: u64,
    /// Whether the scenario should end with exit.
    pub should_end: bool,
    /// Whether the scenario should end with sleep.
    pub should_sleep: bool,
    /// Whether the scenario should be valid (no validation errors).
    pub should_be_valid: bool,
}

#[allow(dead_code)]
impl OutputThreadMockExpectation {
    /// Create a new expectation with all fields.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        expected_final_action: OutputThreadRenderAction,
        min_steps: usize,
        max_steps: usize,
        min_rendered_frames: u64,
        min_silence_frames: u64,
        should_end: bool,
        should_sleep: bool,
        should_be_valid: bool,
    ) -> Self {
        Self {
            expected_final_action,
            min_steps,
            max_steps,
            min_rendered_frames,
            min_silence_frames,
            should_end,
            should_sleep,
            should_be_valid,
        }
    }

    /// Whether this expectation requires an exit as final action.
    pub(crate) fn expects_exit(&self) -> bool {
        self.should_end && self.expected_final_action == OutputThreadRenderAction::Exit
    }

    /// Whether this expectation requires sleep as final action.
    pub(crate) fn expects_sleep(&self) -> bool {
        self.should_sleep && self.expected_final_action == OutputThreadRenderAction::Sleep
    }

    /// Whether the given step count falls within [min_steps, max_steps].
    pub(crate) fn allows_step_count(&self, steps: usize) -> bool {
        steps >= self.min_steps && steps <= self.max_steps
    }

    /// Whether the given rendered frames meet the minimum.
    pub(crate) fn allows_rendered_frames(&self, frames: u64) -> bool {
        frames >= self.min_rendered_frames
    }

    /// Whether the given silence frames meet the minimum.
    pub(crate) fn allows_silence_frames(&self, frames: u64) -> bool {
        frames >= self.min_silence_frames
    }
}
