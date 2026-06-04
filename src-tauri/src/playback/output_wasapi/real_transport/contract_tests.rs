use super::output_thread_real_transport_contract::*;

#[test]
fn contract_only_stage_is_contract_only() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(c.is_contract_only());
}

#[test]
fn contract_only_is_not_transport_started() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.is_transport_started());
}

#[test]
fn contract_only_has_no_sender() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.has_sender);
}

#[test]
fn contract_only_has_no_receiver() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.has_receiver);
}

#[test]
fn contract_only_has_no_worker() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.has_worker);
}

#[test]
fn contract_only_has_no_thread_handle() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.has_thread_handle);
}

#[test]
fn contract_only_has_no_device_boundary() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(!c.has_device_boundary);
}

#[test]
fn contract_only_has_no_runtime_primitives() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert!(c.has_no_runtime_primitives());
}

#[test]
fn unsupported_reasons_include_no_sender() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert!(reasons.contains(&OutputThreadRealTransportUnsupportedReason::NoSender));
}

#[test]
fn unsupported_reasons_include_no_receiver() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert!(reasons.contains(&OutputThreadRealTransportUnsupportedReason::NoReceiver));
}

#[test]
fn unsupported_reasons_include_no_worker() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert!(reasons.contains(&OutputThreadRealTransportUnsupportedReason::NoWorker));
}

#[test]
fn unsupported_reasons_include_no_thread() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert!(reasons.contains(&OutputThreadRealTransportUnsupportedReason::NoThread));
}

#[test]
fn unsupported_reasons_include_no_device() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert!(reasons.contains(&OutputThreadRealTransportUnsupportedReason::NoDevice));
}

#[test]
fn unsupported_reasons_array_has_exactly_five() {
    let c = OutputThreadRealTransportContract::contract_only();
    let reasons = c.unsupported_reasons();
    assert_eq!(reasons.len(), 5);
}

#[test]
fn owner_role_defaults_to_report_only() {
    let c = OutputThreadRealTransportContract::contract_only();
    assert_eq!(c.owner_role, OutputThreadRealTransportOwnerRole::ReportOnly);
}

#[test]
fn stage_names_do_not_imply_playback_ready() {
    let c = OutputThreadRealTransportContract::contract_only();
    let name = format!("{:?}", c.stage);
    assert!(!name.contains("Ready"));
    assert!(!name.contains("Active"));
    assert!(!name.contains("Playing"));
    assert!(!name.contains("Running"));
}

#[test]
fn contract_debug_shows_all_fields() {
    let c = OutputThreadRealTransportContract::contract_only();
    let debug = format!("{:?}", c);
    assert!(debug.contains("ContractOnly"));
    assert!(debug.contains("ReportOnly"));
    assert!(debug.contains("has_sender: false"));
    assert!(debug.contains("has_receiver: false"));
    assert!(debug.contains("has_worker: false"));
    assert!(debug.contains("has_thread_handle: false"));
    assert!(debug.contains("has_device_boundary: false"));
}
