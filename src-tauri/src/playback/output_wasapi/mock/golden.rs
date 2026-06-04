use super::output_thread_mock_expected_behavior::expectation_for_name;
use super::output_thread_mock_scenario_result::OutputThreadMockScenarioResult;

/// Errors from golden behavior validation.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadMockGoldenError {
    /// No expectation found for scenario name.
    MissingExpectation,
    /// Step count outside expected range.
    StepCountOutOfRange,
    /// Rendered frames below minimum.
    RenderedFramesTooLow,
    /// Silence frames below minimum.
    SilenceFramesTooLow,
    /// Should-end mismatch.
    ExpectedExitMismatch,
    /// Should-sleep mismatch.
    ExpectedSleepMismatch,
    /// Validity mismatch.
    ValidityMismatch,
    /// Final action mismatch.
    FinalActionMismatch,
}

/// Result of golden behavior validation.
#[allow(dead_code)]
pub(crate) type OutputThreadMockGoldenResult = Result<(), OutputThreadMockGoldenError>;

/// Validate a scenario result against its expected golden behavior.
#[allow(dead_code)]
pub(crate) fn validate_golden_behavior(
    result: &OutputThreadMockScenarioResult,
) -> OutputThreadMockGoldenResult {
    let expectation =
        expectation_for_name(result.name).ok_or(OutputThreadMockGoldenError::MissingExpectation)?;

    // Step count check
    if !expectation.allows_step_count(result.steps()) {
        return Err(OutputThreadMockGoldenError::StepCountOutOfRange);
    }

    // Rendered frames check
    if !expectation.allows_rendered_frames(result.rendered_frames()) {
        return Err(OutputThreadMockGoldenError::RenderedFramesTooLow);
    }

    // Silence frames check
    if !expectation.allows_silence_frames(result.silence_frames()) {
        return Err(OutputThreadMockGoldenError::SilenceFramesTooLow);
    }

    // Should-end check
    if expectation.should_end != result.ended() {
        return Err(OutputThreadMockGoldenError::ExpectedExitMismatch);
    }

    // Should-sleep check
    if expectation.should_sleep != result.slept() {
        return Err(OutputThreadMockGoldenError::ExpectedSleepMismatch);
    }

    // Validity check
    if expectation.should_be_valid != result.is_valid() {
        return Err(OutputThreadMockGoldenError::ValidityMismatch);
    }

    // Final action check
    let final_action = result.sequence.last().map(|s| s.plan.action);
    match final_action {
        Some(action) => {
            if action != expectation.expected_final_action {
                return Err(OutputThreadMockGoldenError::FinalActionMismatch);
            }
        }
        None => {
            // No steps at all — treat as step count issue
            return Err(OutputThreadMockGoldenError::StepCountOutOfRange);
        }
    }

    Ok(())
}
