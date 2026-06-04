use super::output_thread_runtime_queue_owner_contract::OutputThreadRuntimeQueueOwnerContract;

#[test]
fn contract_only_has_no_entry_collection() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    assert!(c.has_no_entry_collection());
}

#[test]
fn contract_only_has_no_mutable_runtime_queue() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    assert!(c.has_no_mutable_runtime_queue());
}

#[test]
fn contract_only_has_no_output_behavior() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    assert!(c.has_no_output_behavior());
}

#[test]
fn snapshot_projection_contract_allows_snapshot_projection() {
    let c = OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only();
    assert!(c.can_project_snapshot);
    assert!(!c.can_project_bridge_input);
}

#[test]
fn bridge_input_projection_contract_allows_bridge_input_projection() {
    let c = OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only();
    assert!(c.can_project_snapshot);
    assert!(c.can_project_bridge_input);
}

#[test]
fn contract_never_allows_bridge_result_apply() {
    let contracts = [
        OutputThreadRuntimeQueueOwnerContract::contract_only(),
        OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only(),
        OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only(),
    ];
    for c in contracts {
        assert!(
            !c.can_apply_bridge_result,
            "contract at stage {:?} allows bridge result apply",
            c.stage
        );
    }
}

#[test]
fn contract_never_connects_adapter() {
    let contracts = [
        OutputThreadRuntimeQueueOwnerContract::contract_only(),
        OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only(),
        OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only(),
    ];
    for c in contracts {
        assert!(
            !c.adapter_connected,
            "contract at stage {:?} connects adapter",
            c.stage
        );
    }
}

#[test]
fn contract_never_connects_runner() {
    let contracts = [
        OutputThreadRuntimeQueueOwnerContract::contract_only(),
        OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only(),
        OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only(),
    ];
    for c in contracts {
        assert!(
            !c.runner_connected,
            "contract at stage {:?} connects runner",
            c.stage
        );
    }
}
