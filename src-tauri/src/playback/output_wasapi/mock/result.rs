use super::super::output_thread_plan_projection::OutputThreadStatsProjection;
use super::super::output_thread_plan_validation::OutputThreadPlanValidationError;
use super::super::output_thread_render_plan::OutputThreadRenderPlan;

/// Result of a single mock harness step.
///
/// Pure data — does not execute the plan or mutate real stats.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadMockStepResult {
    /// The plan that was computed.
    pub plan: OutputThreadRenderPlan,
    /// The projected stats delta.
    pub projection: OutputThreadStatsProjection,
    /// Validation error, if any.
    pub validation_error: Option<OutputThreadPlanValidationError>,
}

#[allow(dead_code)]
impl OutputThreadMockStepResult {
    /// Whether the step passed all validations.
    pub(crate) fn is_valid(self) -> bool {
        self.validation_error.is_none()
    }

    /// Whether this step should exit the loop.
    pub(crate) fn should_exit(self) -> bool {
        self.projection.should_exit
    }

    /// Whether this step should sleep.
    pub(crate) fn should_sleep(self) -> bool {
        self.projection.should_sleep
    }
}
