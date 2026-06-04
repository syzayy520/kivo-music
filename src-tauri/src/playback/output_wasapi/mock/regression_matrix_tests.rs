use super::output_thread_mock_regression_matrix::*;

#[test]
fn regression_matrix_runs_all_scenarios() {
    let matrix = run_mock_regression_matrix();
    assert_eq!(matrix.len(), 10);
}

#[test]
fn regression_matrix_has_complete_coverage() {
    let matrix = run_mock_regression_matrix();
    assert!(matrix.coverage_complete());
}

#[test]
fn regression_matrix_succeeds() {
    let matrix = run_mock_regression_matrix();
    assert!(
        matrix.is_success(),
        "failed: {:?}",
        matrix
            .results
            .iter()
            .filter(|r| !r.is_success())
            .collect::<Vec<_>>()
    );
}

#[test]
fn regression_matrix_reports_successful_count() {
    let matrix = run_mock_regression_matrix();
    assert_eq!(matrix.successful_count(), 10);
}

#[test]
fn regression_matrix_reports_failed_count() {
    let matrix = run_mock_regression_matrix();
    assert_eq!(matrix.failed_count(), 0);
}

#[test]
fn regression_matrix_is_not_empty() {
    let matrix = run_mock_regression_matrix();
    assert!(!matrix.is_empty());
}
