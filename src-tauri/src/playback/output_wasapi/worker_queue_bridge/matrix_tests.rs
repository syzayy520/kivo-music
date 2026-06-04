use super::matrix::{
    worker_queue_bridge_scenario_matrix, OutputThreadWorkerQueueBridgeScenarioKind,
};

#[test]
fn matrix_contains_six_scenarios() {
    let matrix = worker_queue_bridge_scenario_matrix();
    assert_eq!(matrix.len(), 6);
}

#[test]
fn matrix_names_are_stable() {
    let matrix = worker_queue_bridge_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "start_intent",
            "stop_intent",
            "close_intent",
            "reset_device_intent",
            "full_queue",
            "closed_queue",
        ]
    );
}

#[test]
fn matrix_all_expect_queue_bridge() {
    let matrix = worker_queue_bridge_scenario_matrix();
    for scenario in matrix {
        assert!(
            scenario.expects_queue_bridge,
            "scenario {:?} does not expect queue bridge",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_output_behavior() {
    let matrix = worker_queue_bridge_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_output_behavior,
            "scenario {:?} expects output behavior",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_runtime_loop_pass() {
    let matrix = worker_queue_bridge_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_runtime_loop_pass,
            "scenario {:?} expects runtime loop pass",
            scenario.name
        );
    }
}

#[test]
fn matrix_contains_start_or_equivalent() {
    let matrix = worker_queue_bridge_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerQueueBridgeScenarioKind::StartIntent));
}

#[test]
fn matrix_contains_closed_queue_case() {
    let matrix = worker_queue_bridge_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerQueueBridgeScenarioKind::ClosedQueue));
}

#[test]
fn matrix_contains_full_queue_case() {
    let matrix = worker_queue_bridge_scenario_matrix();
    assert!(matrix
        .iter()
        .any(|s| s.kind == OutputThreadWorkerQueueBridgeScenarioKind::FullQueue));
}
