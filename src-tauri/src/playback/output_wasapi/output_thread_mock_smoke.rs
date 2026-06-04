use super::output_thread_mock_regression_matrix::run_mock_regression_matrix;

/// Run the full mock smoke suite.
///
/// Pure mock only — no thread, no external state.
#[allow(dead_code)]
pub(crate) fn run_mock_smoke_suite() -> bool {
    let matrix = run_mock_regression_matrix();
    matrix.is_success() && matrix.coverage_complete()
}
