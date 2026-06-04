use super::output_thread_worker_loop_matrix::{
    worker_loop_scenario_matrix, OutputThreadWorkerLoopScenarioKind,
};

#[test]
fn matrix_contains_five_scenarios() {
    let matrix = worker_loop_scenario_matrix();
    assert_eq!(matrix.len(), 5);
}

#[test]
fn matrix_names_are_stable() {
    let matrix = worker_loop_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "empty_channel",
            "single_runtime_intent",
            "close_transport",
            "zero_step_budget",
            "one_step_budget",
        ]
    );
}

#[test]
fn matrix_contains_empty_channel() {
    let matrix = worker_loop_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerLoopScenarioKind::EmptyChannel));
}

#[test]
fn matrix_contains_close_transport() {
    let matrix = worker_loop_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerLoopScenarioKind::CloseTransport));
}

#[test]
fn matrix_contains_zero_step_budget() {
    let matrix = worker_loop_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerLoopScenarioKind::ZeroStepBudget));
}

#[test]
fn matrix_cases_do_not_imply_worker_thread() {
    let matrix = worker_loop_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.name.contains("thread"),
            "scenario {:?}",
            scenario.name
        );
        assert!(
            !scenario.name.contains("spawn"),
            "scenario {:?}",
            scenario.name
        );
    }
}
