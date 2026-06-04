use super::output_thread_mock_coverage::OutputThreadMockCoverage;
use super::output_thread_mock_regression::{run_mock_regression, OutputThreadMockRegressionResult};
use super::output_thread_mock_scenario_matrix::{all_mock_scenarios, scenario_names};

/// Result of running all mock regressions.
///
/// Pure in-memory — no thread, no external state.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadMockRegressionMatrix {
    /// Individual regression results.
    pub results: Vec<OutputThreadMockRegressionResult>,
    /// Scenario coverage tracker.
    pub coverage: OutputThreadMockCoverage,
}

#[allow(dead_code)]
impl OutputThreadMockRegressionMatrix {
    /// Number of regression results.
    pub(crate) fn len(&self) -> usize {
        self.results.len()
    }

    /// Whether the matrix has no results.
    pub(crate) fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Whether all regressions passed.
    pub(crate) fn is_success(&self) -> bool {
        self.results.iter().all(|r| r.is_success())
    }

    /// Number of failed regressions.
    pub(crate) fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.is_success()).count()
    }

    /// Number of successful regressions.
    pub(crate) fn successful_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_success()).count()
    }

    /// Whether all 10 scenarios are covered.
    pub(crate) fn coverage_complete(&self) -> bool {
        self.coverage.is_complete()
    }
}

/// Run all mock regressions and build the matrix.
#[allow(dead_code)]
pub(crate) fn run_mock_regression_matrix() -> OutputThreadMockRegressionMatrix {
    let scenarios = all_mock_scenarios();
    let results = scenarios.into_iter().map(run_mock_regression).collect();
    let coverage = OutputThreadMockCoverage::from_names(&scenario_names());
    OutputThreadMockRegressionMatrix { results, coverage }
}
