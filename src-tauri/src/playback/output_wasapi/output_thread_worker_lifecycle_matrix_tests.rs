use super::output_thread_worker_lifecycle_matrix::*;
use super::output_thread_worker_lifecycle_plan::plan_worker_lifecycle;

#[test]
fn matrix_contains_five_scenarios() {
    let matrix = worker_lifecycle_scenario_matrix();
    assert_eq!(matrix.len(), 5);
}

#[test]
fn matrix_names_are_stable() {
    let matrix = worker_lifecycle_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "contract_only_no_request",
            "contract_only_stop_request",
            "contract_only_close_transport",
            "stopped_stop_request",
            "failed_stop_request",
        ]
    );
}

#[test]
fn matrix_runs_all_scenarios() {
    let matrix = worker_lifecycle_scenario_matrix();
    for scenario in matrix {
        let decision = plan_worker_lifecycle(scenario.input);
        // All scenarios must produce a valid decision
        let _ = decision.next_stage;
        let _ = decision.outcome;
        let _ = decision.should_continue_scaffold;
    }
}

#[test]
fn matrix_contains_close_transport_case() {
    let matrix = worker_lifecycle_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerLifecycleScenarioKind::ContractOnlyCloseTransport));
}

#[test]
fn matrix_contains_stop_request_case() {
    let matrix = worker_lifecycle_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerLifecycleScenarioKind::ContractOnlyStopRequest));
}

#[test]
fn matrix_cases_do_not_imply_worker_running() {
    let matrix = worker_lifecycle_scenario_matrix();
    for scenario in matrix {
        let decision = plan_worker_lifecycle(scenario.input);
        assert!(!decision.next_stage.has_live_worker());
    }
}
