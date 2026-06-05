use super::smoke::run_mock_smoke_suite;

#[test]
fn mock_smoke_suite_passes() {
    assert!(run_mock_smoke_suite());
}

#[test]
fn mock_smoke_suite_is_deterministic() {
    let first = run_mock_smoke_suite();
    let second = run_mock_smoke_suite();
    assert_eq!(first, second);
}
