use super::contract::OutputThreadRealTransportContract;
use super::status::*;

#[test]
fn report_from_contract_is_not_created() {
    let contract = OutputThreadRealTransportContract::contract_only();
    let report = OutputThreadRealTransportStatusReport::from_contract(contract);
    assert_eq!(report.status, OutputThreadRealTransportStatus::NotCreated);
}

#[test]
fn report_from_contract_has_no_worker_or_thread() {
    let contract = OutputThreadRealTransportContract::contract_only();
    let report = OutputThreadRealTransportStatusReport::from_contract(contract);
    assert!(report.has_no_worker_or_thread());
}

#[test]
fn channel_created_report_has_sender_and_receiver() {
    let report = OutputThreadRealTransportStatusReport::channel_created();
    assert!(report.has_sender);
    assert!(report.has_receiver);
}

#[test]
fn channel_created_report_has_no_worker_or_thread() {
    let report = OutputThreadRealTransportStatusReport::channel_created();
    assert!(report.has_no_worker_or_thread());
}

#[test]
fn closed_report_is_closed() {
    let report = OutputThreadRealTransportStatusReport::closed();
    assert_eq!(report.status, OutputThreadRealTransportStatus::Closed);
}

#[test]
fn closed_report_has_no_worker_or_thread() {
    let report = OutputThreadRealTransportStatusReport::closed();
    assert!(report.has_no_worker_or_thread());
}

#[test]
fn status_names_do_not_imply_playback_ready() {
    let variants = [
        OutputThreadRealTransportStatus::NotCreated,
        OutputThreadRealTransportStatus::ChannelCreated,
        OutputThreadRealTransportStatus::Closed,
    ];
    for v in variants {
        let name = format!("{:?}", v);
        assert!(!name.contains("Ready"), "variant {:?} contains Ready", v);
        assert!(!name.contains("Active"), "variant {:?} contains Active", v);
        assert!(!name.contains("Playing"), "variant {:?} contains Playing", v);
        assert!(!name.contains("Running"), "variant {:?} contains Running", v);
    }
}

#[test]
fn channel_created_report_is_channel_created() {
    let report = OutputThreadRealTransportStatusReport::channel_created();
    assert!(report.is_channel_created());
}

#[test]
fn closed_report_is_not_channel_created() {
    let report = OutputThreadRealTransportStatusReport::closed();
    assert!(!report.is_channel_created());
}

#[test]
fn report_debug_shows_status() {
    let report = OutputThreadRealTransportStatusReport::channel_created();
    let debug = format!("{:?}", report);
    assert!(debug.contains("ChannelCreated"));
}
