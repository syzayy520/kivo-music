use super::scenario_matrix::{all_mock_scenarios, scenario_names};
use super::scenario_runner::run_mock_scenario;

#[test]
fn matrix_contains_all_expected_scenarios() {
    let scenarios = all_mock_scenarios();
    assert_eq!(scenarios.len(), 10);
}

#[test]
fn matrix_order_is_stable() {
    let scenarios = all_mock_scenarios();
    assert_eq!(scenarios[0].name, "normal_audio");
    assert_eq!(scenarios[1].name, "empty_running");
    assert_eq!(scenarios[2].name, "no_capacity");
    assert_eq!(scenarios[3].name, "shutdown_requested");
    assert_eq!(scenarios[4].name, "paused_empty");
    assert_eq!(scenarios[5].name, "paused_with_frames");
    assert_eq!(scenarios[6].name, "flush_empty");
    assert_eq!(scenarios[7].name, "closed_empty");
    assert_eq!(scenarios[8].name, "closed_with_remaining");
    assert_eq!(scenarios[9].name, "non_running");
}

#[test]
fn scenario_names_match_matrix() {
    let names = scenario_names();
    let scenarios = all_mock_scenarios();
    assert_eq!(names.len(), scenarios.len());
    for (name, scenario) in names.iter().zip(scenarios.iter()) {
        assert_eq!(*name, scenario.name);
    }
}

#[test]
fn matrix_scenarios_are_bounded() {
    for scenario in all_mock_scenarios() {
        assert!(scenario.is_bounded(), "{} should be bounded", scenario.name);
    }
}

#[test]
fn matrix_scenarios_have_unique_names() {
    let names = scenario_names();
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(names.len(), sorted.len());
}

#[test]
fn all_matrix_scenarios_run_without_validation_errors() {
    let scenarios = all_mock_scenarios();
    for scenario in scenarios {
        let result = run_mock_scenario(scenario);
        assert!(
            result.is_valid(),
            "{} should have no validation errors",
            result.name
        );
    }
}
