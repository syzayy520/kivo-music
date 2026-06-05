use super::super::output_thread_consumer_plan::plan_consumer_step;
use super::super::output_thread_control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::renderer::OutputThreadMockRenderer;
use super::result::OutputThreadMockStepResult;
use super::super::output_thread_plan_invariants::validate_render_plan_shape;
use super::super::output_thread_plan_projection::{project_stats_from_plan, OutputThreadStatsProjection};
use super::super::output_thread_transition_validation::validate_plan_for_state;

/// Run a single mock harness step.
///
/// Pure function — no buffer consumption, no thread, no WASAPI.
#[allow(dead_code)]
pub(crate) fn plan_mock_step(
    buffer: OutputThreadMockBuffer,
    renderer: OutputThreadMockRenderer,
    control: OutputThreadControlSnapshot,
) -> OutputThreadMockStepResult {
    let snapshot = buffer.snapshot();
    let free_frames = renderer.free_frames();
    let plan = plan_consumer_step(snapshot, free_frames, control);
    let shape_result = validate_render_plan_shape(plan);
    let transition_result = validate_plan_for_state(snapshot, control, plan);
    let validation_error = shape_result.err().or(transition_result.err());
    let projection = project_stats_from_plan(plan);
    OutputThreadMockStepResult {
        plan,
        projection,
        validation_error,
    }
}

/// Get only the stats projection from a mock step.
#[allow(dead_code)]
pub(crate) fn plan_mock_step_projection_only(
    buffer: OutputThreadMockBuffer,
    renderer: OutputThreadMockRenderer,
    control: OutputThreadControlSnapshot,
) -> OutputThreadStatsProjection {
    plan_mock_step(buffer, renderer, control).projection
}
