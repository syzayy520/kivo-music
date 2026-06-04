use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_mock_buffer::OutputThreadMockBuffer;
use super::output_thread_mock_renderer::OutputThreadMockRenderer;

/// Input parameters for a single mock scenario.
///
/// Pure data — does not execute the scenario.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadMockScenario {
    /// Human-readable scenario name.
    pub name: &'static str,
    /// Mock buffer state.
    pub buffer: OutputThreadMockBuffer,
    /// Mock renderer capacity.
    pub renderer: OutputThreadMockRenderer,
    /// Control snapshot (state, paused, shutdown, flush).
    pub control: OutputThreadControlSnapshot,
    /// Maximum number of steps to run.
    pub max_steps: usize,
}

#[allow(dead_code)]
impl OutputThreadMockScenario {
    /// Create a new scenario with explicit parameters.
    pub(crate) fn new(
        name: &'static str,
        buffer: OutputThreadMockBuffer,
        renderer: OutputThreadMockRenderer,
        control: OutputThreadControlSnapshot,
        max_steps: usize,
    ) -> Self {
        Self {
            name,
            buffer,
            renderer,
            control,
            max_steps,
        }
    }

    /// Whether the scenario has a bounded number of steps.
    pub(crate) fn is_bounded(&self) -> bool {
        self.max_steps > 0
    }

    /// Whether the renderer has capacity.
    pub(crate) fn has_capacity(&self) -> bool {
        self.renderer.has_capacity()
    }

    /// Maximum number of steps.
    pub(crate) fn max_steps(&self) -> usize {
        self.max_steps
    }
}
