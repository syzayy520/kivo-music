use super::golden::validate_golden_behavior;
use super::regression::OutputThreadMockRegressionResult;
use super::regression_matrix::OutputThreadMockRegressionMatrix;
use super::scenario_result::OutputThreadMockScenarioResult;

/// Whether a scenario result passes golden validation.
#[allow(dead_code)]
pub(crate) fn result_passes_golden(result: &OutputThreadMockScenarioResult) -> bool {
    validate_golden_behavior(result).is_ok()
}

/// Whether a regression result succeeded.
#[allow(dead_code)]
pub(crate) fn regression_succeeded(result: &OutputThreadMockRegressionResult) -> bool {
    result.is_success()
}

/// Whether all regressions in the matrix succeeded.
#[allow(dead_code)]
pub(crate) fn matrix_succeeded(matrix: &OutputThreadMockRegressionMatrix) -> bool {
    matrix.is_success()
}

/// Whether the matrix has complete scenario coverage.
#[allow(dead_code)]
pub(crate) fn matrix_has_complete_coverage(matrix: &OutputThreadMockRegressionMatrix) -> bool {
    matrix.coverage_complete()
}
