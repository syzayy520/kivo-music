use super::output_thread_worker_runtime_matrix::{
    worker_runtime_scenario_matrix, OutputThreadWorkerRuntimeScenarioKind,
};

#[test]
fn matrix_contains_four_scenarios() {
    let matrix = worker_runtime_scenario_matrix();
    assert_eq!(matrix.len(), 4);
}

#[test]
fn matrix_names_are_stable() {
    let matrix = worker_runtime_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "no_command",
            "runtime_intent_observed",
            "shutdown_intent_observed",
            "worker_transport_closed",
        ]
    );
}

#[test]
fn matrix_all_expect_no_queue_bridge() {
    let matrix = worker_runtime_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_queue_bridge,
            "scenario {:?} expects queue bridge",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_output_behavior() {
    let matrix = worker_runtime_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_output_behavior,
            "scenario {:?} expects output behavior",
            scenario.name
        );
    }
}

#[test]
fn matrix_contains_transport_closed() {
    let matrix = worker_runtime_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerRuntimeScenarioKind::WorkerTransportClosed));
}

#[test]
fn matrix_contains_runtime_intent_observed() {
    let matrix = worker_runtime_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerRuntimeScenarioKind::RuntimeIntentObserved));
}
