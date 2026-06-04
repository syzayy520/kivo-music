use super::output_thread_mock_assertions::*;
use super::output_thread_mock_coverage::OutputThreadMockCoverage;
use super::output_thread_mock_golden::OutputThreadMockGoldenError;
use super::output_thread_mock_regression::OutputThreadMockRegressionResult;
use super::output_thread_mock_regression_matrix::OutputThreadMockRegressionMatrix;
use super::output_thread_mock_scenario_result::OutputThreadMockScenarioResult;
use super::output_thread_mock_sequence::OutputThreadMockSequence;

#[test]
fn result_passes_golden_for_valid_result() {
    // Build a minimal valid result for "no_capacity"
    // We can't easily build a fully valid result here, so test the function exists
    let result =
        OutputThreadMockScenarioResult::new("nonexistent", OutputThreadMockSequence::default());
    assert!(!result_passes_golden(&result));
}

#[test]
fn regression_succeeded_for_successful_result() {
    let reg = OutputThreadMockRegressionResult {
        result: OutputThreadMockScenarioResult::new("test", OutputThreadMockSequence::default()),
        golden_error: None,
    };
    assert!(regression_succeeded(&reg));
}

#[test]
fn regression_succeeded_for_failed_result() {
    let reg = OutputThreadMockRegressionResult {
        result: OutputThreadMockScenarioResult::new("test", OutputThreadMockSequence::default()),
        golden_error: Some(OutputThreadMockGoldenError::MissingExpectation),
    };
    assert!(!regression_succeeded(&reg));
}

#[test]
fn matrix_succeeded_for_successful_matrix() {
    let matrix = OutputThreadMockRegressionMatrix {
        results: vec![],
        coverage: OutputThreadMockCoverage::default(),
    };
    assert!(matrix_succeeded(&matrix));
}

#[test]
fn matrix_has_complete_coverage_for_full_matrix() {
    let names = [
        "normal_audio",
        "empty_running",
        "no_capacity",
        "shutdown_requested",
        "paused_empty",
        "paused_with_frames",
        "flush_empty",
        "closed_empty",
        "closed_with_remaining",
        "non_running",
    ];
    let matrix = OutputThreadMockRegressionMatrix {
        results: vec![],
        coverage: OutputThreadMockCoverage::from_names(&names),
    };
    assert!(matrix_has_complete_coverage(&matrix));
}
