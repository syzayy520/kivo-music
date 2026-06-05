use super::super::output_thread_control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::harness::plan_mock_step;
use super::renderer::OutputThreadMockRenderer;
use super::result::OutputThreadMockStepResult;
use super::super::output_thread_render_plan::OutputThreadRenderAction;

/// Sequence of mock harness steps.
///
/// Pure in-memory container — no thread, no channel.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadMockSequence {
    pub steps: Vec<OutputThreadMockStepResult>,
}

#[allow(dead_code)]
impl OutputThreadMockSequence {
    /// Append a step result.
    pub(crate) fn push(mut self, result: OutputThreadMockStepResult) -> Self {
        self.steps.push(result);
        self
    }

    /// Number of steps.
    pub(crate) fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether the sequence is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Last step result.
    pub(crate) fn last(&self) -> Option<&OutputThreadMockStepResult> {
        self.steps.last()
    }

    /// Total rendered frames across all steps.
    pub(crate) fn total_rendered_frames(&self) -> u64 {
        self.steps
            .iter()
            .map(|s| s.projection.rendered_frames_delta)
            .sum()
    }

    /// Total silence frames across all steps.
    pub(crate) fn total_silence_frames(&self) -> u64 {
        self.steps
            .iter()
            .map(|s| s.projection.silence_filled_frames_delta)
            .sum()
    }

    /// Whether any step had a validation error.
    pub(crate) fn has_validation_errors(&self) -> bool {
        self.steps.iter().any(|s| s.validation_error.is_some())
    }
}

/// Run a mock consumer sequence for up to max_steps.
///
/// Pure function — no thread, no sleep, no external state.
#[allow(dead_code)]
pub(crate) fn run_mock_sequence(
    mut buffer: OutputThreadMockBuffer,
    renderer: OutputThreadMockRenderer,
    control: OutputThreadControlSnapshot,
    max_steps: usize,
) -> OutputThreadMockSequence {
    let mut sequence = OutputThreadMockSequence::default();

    for _ in 0..max_steps {
        let result = plan_mock_step(buffer, renderer, control);
        let should_exit = result.should_exit();
        let should_sleep = result.should_sleep();
        let plan = result.plan;

        sequence = sequence.push(result);

        if should_exit {
            break;
        }

        // Consume audio frames from the mock buffer.
        if plan.action == OutputThreadRenderAction::RenderAudio {
            buffer = buffer.consume(plan.frames_to_read);
        }

        if should_sleep {
            break;
        }
    }

    sequence
}
