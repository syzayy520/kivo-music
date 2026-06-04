use super::output_thread_runtime_queue_owner_matrix::{
    runtime_queue_owner_scenario_matrix, OutputThreadRuntimeQueueOwnerScenarioKind,
};

#[test]
fn matrix_names_are_stable() {
    let matrix = runtime_queue_owner_scenario_matrix();
    let names: Vec<&str> = matrix.iter().map(|s| s.name).collect();
    assert_eq!(
        names,
        vec![
            "contract_only",
            "snapshot_projection_allowed",
            "bridge_input_projection_allowed",
            "bridge_result_apply_denied",
            "entry_collection_denied",
            "adapter_connection_denied",
            "runner_connection_denied",
        ]
    );
}

#[test]
fn matrix_all_expect_no_entry_collection() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_entry_collection,
            "scenario {:?} expects entry collection",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_mutable_queue() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_mutable_queue,
            "scenario {:?} expects mutable queue",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_bridge_result_apply() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_bridge_result_apply,
            "scenario {:?} expects bridge result apply",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_adapter_connection() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_adapter_connection,
            "scenario {:?} expects adapter connection",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_runner_connection() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_runner_connection,
            "scenario {:?} expects runner connection",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_output_behavior() {
    let matrix = runtime_queue_owner_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_output_behavior,
            "scenario {:?} expects output behavior",
            scenario.name
        );
    }
}

#[test]
fn matrix_contains_snapshot_projection_case() {
    let matrix = runtime_queue_owner_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadRuntimeQueueOwnerScenarioKind::SnapshotProjectionAllowed));
}

#[test]
fn matrix_contains_bridge_input_projection_case() {
    let matrix = runtime_queue_owner_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadRuntimeQueueOwnerScenarioKind::BridgeInputProjectionAllowed));
}
