use super::output_thread_runtime_queue_owner_contract::OutputThreadRuntimeQueueOwnerContract;
use super::output_thread_runtime_queue_owner_report::OutputThreadRuntimeQueueOwnerReport;

#[test]
fn empty_report_has_no_output_behavior() {
    let r = OutputThreadRuntimeQueueOwnerReport::empty();
    assert!(r.has_no_output_behavior());
}

#[test]
fn record_contract_increments_contract_count() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert_eq!(r.contracts_created, 1);
}

#[test]
fn report_records_snapshot_projection_allowed() {
    let c = OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.snapshot_projection_allowed);
    assert!(!r.bridge_input_projection_allowed);
}

#[test]
fn report_records_bridge_input_projection_allowed() {
    let c = OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.snapshot_projection_allowed);
    assert!(r.bridge_input_projection_allowed);
}

#[test]
fn report_never_allows_bridge_result_apply() {
    let contracts = [
        OutputThreadRuntimeQueueOwnerContract::contract_only(),
        OutputThreadRuntimeQueueOwnerContract::snapshot_projection_only(),
        OutputThreadRuntimeQueueOwnerContract::bridge_input_projection_only(),
    ];
    for c in contracts {
        let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
        assert!(
            !r.bridge_result_apply_allowed,
            "report allows bridge result apply for contract"
        );
    }
}

#[test]
fn report_never_owns_entry_collection() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.has_no_entry_collection());
}

#[test]
fn report_never_owns_mutable_queue() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.has_no_mutable_queue());
}

#[test]
fn report_never_connects_adapter() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.has_no_adapter_connection());
}

#[test]
fn report_never_connects_runner() {
    let c = OutputThreadRuntimeQueueOwnerContract::contract_only();
    let r = OutputThreadRuntimeQueueOwnerReport::empty().record_contract(c);
    assert!(r.has_no_runner_connection());
}
