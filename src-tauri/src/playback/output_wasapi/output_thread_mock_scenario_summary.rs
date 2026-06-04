use super::output_thread_mock_sequence::OutputThreadMockSequence;

/// Summary of a mock scenario execution.
///
/// Pure data — derived from a completed sequence.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadMockScenarioSummary {
    /// Number of steps executed.
    pub steps: usize,
    /// Total rendered frames (audio + silence).
    pub rendered_frames: u64,
    /// Total silence frames filled.
    pub silence_frames: u64,
    /// Whether any step had a validation error.
    pub has_errors: bool,
    /// Whether the sequence ended with an exit plan.
    pub ended: bool,
    /// Whether the sequence ended with a sleep plan.
    pub slept: bool,
}

#[allow(dead_code)]
impl OutputThreadMockScenarioSummary {
    /// Build a summary from a completed sequence.
    pub(crate) fn from_sequence(sequence: &OutputThreadMockSequence) -> Self {
        let last = sequence.last();
        Self {
            steps: sequence.len(),
            rendered_frames: sequence.total_rendered_frames(),
            silence_frames: sequence.total_silence_frames(),
            has_errors: sequence.has_validation_errors(),
            ended: match last {
                Some(s) => s.should_exit(),
                None => false,
            },
            slept: match last {
                Some(s) => s.should_sleep(),
                None => false,
            },
        }
    }
}
