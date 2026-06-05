use super::scenario_summary::OutputThreadMockScenarioSummary;
use super::sequence::OutputThreadMockSequence;

/// Result of running a mock scenario.
///
/// Pure data — contains the sequence and its summary.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadMockScenarioResult {
    /// Scenario name.
    pub name: &'static str,
    /// The executed sequence.
    pub sequence: OutputThreadMockSequence,
    /// Summary of the sequence.
    pub summary: OutputThreadMockScenarioSummary,
}

#[allow(dead_code)]
impl OutputThreadMockScenarioResult {
    /// Build a result from a scenario name and completed sequence.
    pub(crate) fn new(name: &'static str, sequence: OutputThreadMockSequence) -> Self {
        let summary = OutputThreadMockScenarioSummary::from_sequence(&sequence);
        Self {
            name,
            sequence,
            summary,
        }
    }

    /// Whether the scenario had no validation errors.
    pub(crate) fn is_valid(&self) -> bool {
        !self.summary.has_errors
    }

    /// Number of steps executed.
    pub(crate) fn steps(&self) -> usize {
        self.summary.steps
    }

    /// Total rendered frames.
    pub(crate) fn rendered_frames(&self) -> u64 {
        self.summary.rendered_frames
    }

    /// Total silence frames.
    pub(crate) fn silence_frames(&self) -> u64 {
        self.summary.silence_frames
    }

    /// Whether the sequence ended with an exit plan.
    pub(crate) fn ended(&self) -> bool {
        self.summary.ended
    }

    /// Whether the sequence ended with a sleep plan.
    pub(crate) fn slept(&self) -> bool {
        self.summary.slept
    }
}
