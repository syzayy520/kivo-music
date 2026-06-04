use super::handle_contract::*;
use super::lifecycle::OutputThreadWorkerLifecycleStage;

#[test]
fn contract_only_has_no_handle() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert!(c.has_no_handle());
}

#[test]
fn contract_only_has_no_worker_loop() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert!(c.has_no_worker_loop());
}

#[test]
fn contract_only_cannot_join() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert!(!c.can_join_now());
}

#[test]
fn contract_only_cannot_signal_stop() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert!(!c.can_signal_stop_now());
}

#[test]
fn ownership_defaults_to_not_owned() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert_eq!(c.ownership, OutputThreadWorkerHandleOwnership::NotOwned);
}

#[test]
fn lifecycle_defaults_to_contract_only() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    assert_eq!(c.lifecycle, OutputThreadWorkerLifecycleStage::ContractOnly);
}

#[test]
fn contract_debug_shows_all_fields() {
    let c = OutputThreadWorkerHandleContract::contract_only();
    let debug = format!("{:?}", c);
    assert!(debug.contains("ContractOnly"));
    assert!(debug.contains("NotOwned"));
    assert!(debug.contains("has_handle: false"));
    assert!(debug.contains("has_worker_loop: false"));
}
