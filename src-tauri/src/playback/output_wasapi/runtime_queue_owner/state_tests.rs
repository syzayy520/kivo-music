use super::super::runtime_queue::config::OutputThreadRuntimeQueueConfig;
use super::state::{
    OutputThreadRuntimeQueueOwnerState, OutputThreadRuntimeQueueOwnerTransitionKind,
};
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;

fn make_state() -> OutputThreadRuntimeQueueOwnerState {
    OutputThreadRuntimeQueueOwnerState::contract_only(
        OutputThreadRuntimeQueueConfig::default(),
        OutputThreadRuntimeQueueState::empty(),
    )
}

#[test]
fn state_contract_only_has_no_entry_collection() {
    let s = make_state();
    assert!(s.has_no_entry_collection());
}

#[test]
fn state_contract_only_has_no_mutable_queue() {
    let s = make_state();
    assert!(s.has_no_mutable_runtime_queue());
}

#[test]
fn state_can_enable_snapshot_projection_stage() {
    let s = make_state().enable_snapshot_projection();
    assert_eq!(
        s.stage,
        super::contract::OutputThreadRuntimeQueueOwnerStage::SnapshotProjectionOnly
    );
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::EnabledSnapshotProjection
    );
}

#[test]
fn state_can_enable_bridge_input_projection_stage() {
    let s = make_state().enable_bridge_input_projection();
    assert_eq!(
        s.stage,
        super::contract::OutputThreadRuntimeQueueOwnerStage::BridgeInputProjectionOnly
    );
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::EnabledBridgeInputProjection
    );
}

#[test]
fn state_denies_bridge_result_apply() {
    let s = make_state().deny_bridge_result_apply();
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::BridgeResultApplyDenied
    );
}

#[test]
fn state_denies_entry_collection() {
    let s = make_state().deny_entry_collection();
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::EntryCollectionDenied
    );
}

#[test]
fn state_denies_adapter_connection() {
    let s = make_state().deny_adapter_connection();
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::AdapterConnectionDenied
    );
}

#[test]
fn state_denies_runner_connection() {
    let s = make_state().deny_runner_connection();
    assert_eq!(
        s.last_transition,
        OutputThreadRuntimeQueueOwnerTransitionKind::RunnerConnectionDenied
    );
}

#[test]
fn state_has_no_output_behavior() {
    let s = make_state();
    assert!(s.has_no_output_behavior());
}
