use super::output_thread_runtime_queue_owner_slot_matrix::{
    runtime_queue_owner_slot_scenario_matrix, OutputThreadRuntimeQueueOwnerSlotScenarioKind,
};

#[test]
fn matrix_names_are_stable() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    assert_eq!(matrix[0].name, "empty_slots");
    assert_eq!(matrix[1].name, "single_occupied_slot");
    assert_eq!(matrix[2].name, "full_slots");
    assert_eq!(matrix[3].name, "snapshot_projection");
    assert_eq!(matrix[4].name, "bridge_result_apply_denied");
    assert_eq!(matrix[5].name, "adapter_connection_denied");
    assert_eq!(matrix[6].name, "runner_connection_denied");
}

#[test]
fn matrix_all_expect_fixed_slots() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    for scenario in matrix {
        assert!(
            scenario.expects_fixed_slots,
            "scenario {:?} does not expect fixed slots",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_dynamic_collection() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_dynamic_collection,
            "scenario {:?} expects dynamic collection",
            scenario.name
        );
    }
}

#[test]
fn matrix_all_expect_no_bridge_result_apply() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
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
    let matrix = runtime_queue_owner_slot_scenario_matrix();
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
    let matrix = runtime_queue_owner_slot_scenario_matrix();
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
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    for scenario in matrix {
        assert!(
            !scenario.expects_output_behavior,
            "scenario {:?} expects output behavior",
            scenario.name
        );
    }
}

#[test]
fn matrix_contains_empty_slots_case() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadRuntimeQueueOwnerSlotScenarioKind::EmptySlots));
}

#[test]
fn matrix_contains_full_slots_case() {
    let matrix = runtime_queue_owner_slot_scenario_matrix();
    assert!(matrix.iter().any(|s| s.kind
        == OutputThreadRuntimeQueueOwnerSlotScenarioKind::FullSlots));
}
