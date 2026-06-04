use super::output_thread_worker_runtime_queue_result_matrix::{
    worker_runtime_queue_result_scenario_matrix, OutputThreadWorkerRuntimeQueueResultScenarioKind,
};

#[test]
fn matrix_contains_six_scenarios() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    assert_eq!(matrix.len(), 6);
}

#[test]
fn matrix_names_are_stable() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "accepted_start_from_idle",
            "accepted_stop_from_active",
            "accepted_close_from_active",
            "rejected_from_active",
            "accepted_start_from_active",
            "accepted_stop_from_idle",
        ]
    );
}

#[test]
fn matrix_all_expect_queue_bridge_pass() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    for scenario in matrix {
        assert!(
            scenario.expects_queue_bridge_pass,
            "scenario {:?} does not expect queue bridge pass",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_queue_ownership() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_queue_ownership,
            "scenario {:?} expects queue ownership",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_output_behavior() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_output_behavior,
            "scenario {:?} expects output behavior",
            scenario.name
        );
    }
}

#[test]
fn matrix_contains_rejected_case() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadWorkerRuntimeQueueResultScenarioKind::RejectedFromActive));
}

#[test]
fn matrix_contains_accepted_case() {
    let matrix = worker_runtime_queue_result_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedStartFromIdle));
}
