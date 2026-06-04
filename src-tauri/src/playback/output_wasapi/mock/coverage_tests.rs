use super::output_thread_mock_coverage::OutputThreadMockCoverage;

#[test]
fn empty_coverage_is_incomplete() {
    let c = OutputThreadMockCoverage::default();
    assert!(!c.is_complete());
    assert_eq!(c.covered_count(), 0);
}

#[test]
fn full_coverage_is_complete() {
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
    let c = OutputThreadMockCoverage::from_names(&names);
    assert!(c.is_complete());
    assert_eq!(c.covered_count(), 10);
}

#[test]
fn covered_count_counts_unique_known_names() {
    let c =
        OutputThreadMockCoverage::from_names(&["normal_audio", "normal_audio", "empty_running"]);
    assert_eq!(c.covered_count(), 2);
}

#[test]
fn unknown_names_are_ignored() {
    let c = OutputThreadMockCoverage::from_names(&["normal_audio", "unknown", "empty_running"]);
    assert_eq!(c.covered_count(), 2);
}

#[test]
fn missing_count_reports_remaining() {
    let c = OutputThreadMockCoverage::from_names(&["normal_audio", "empty_running"]);
    assert_eq!(c.missing_count(), 8);
}

#[test]
fn expected_count_is_ten() {
    let c = OutputThreadMockCoverage::default();
    assert_eq!(c.expected_count(), 10);
}
