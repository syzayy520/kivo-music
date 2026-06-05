use super::golden::{validate_golden_behavior, OutputThreadMockGoldenError};
use super::scenario::OutputThreadMockScenario;
use super::scenario_result::OutputThreadMockScenarioResult;
use super::scenario_runner::run_mock_scenario;

/// Result of a mock regression run.
///
/// Combines scenario result with golden validation outcome.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadMockRegressionResult {
    /// The scenario execution result.
    pub result: OutputThreadMockScenarioResult,
    /// Golden validation error, if any.
    pub golden_error: Option<OutputThreadMockGoldenError>,
}

#[allow(dead_code)]
impl OutputThreadMockRegressionResult {
    /// Whether regression passed (no golden error).
    pub(crate) fn is_success(&self) -> bool {
        self.golden_error.is_none()
    }

    /// Scenario name.
    pub(crate) fn name(&self) -> &'static str {
        self.result.name
    }
}

/// Run a mock regression: execute scenario and validate golden behavior.
#[allow(dead_code)]
pub(crate) fn run_mock_regression(
    scenario: OutputThreadMockScenario,
) -> OutputThreadMockRegressionResult {
    let result = run_mock_scenario(scenario);
    let golden_error = validate_golden_behavior(&result).err();
    OutputThreadMockRegressionResult {
        result,
        golden_error,
    }
}
