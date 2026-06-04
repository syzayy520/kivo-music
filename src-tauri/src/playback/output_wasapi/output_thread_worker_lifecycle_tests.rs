use super::output_thread_worker_lifecycle::*;

#[test]
fn contract_only_has_no_live_worker() {
    assert!(!OutputThreadWorkerLifecycleStage::ContractOnly.has_live_worker());
}

#[test]
fn handle_not_started_has_no_live_worker() {
    assert!(!OutputThreadWorkerLifecycleStage::HandleNotStarted.has_live_worker());
}

#[test]
fn stopped_is_terminal() {
    assert!(OutputThreadWorkerLifecycleStage::Stopped.is_terminal());
}

#[test]
fn failed_is_terminal() {
    assert!(OutputThreadWorkerLifecycleStage::Failed.is_terminal());
}

#[test]
fn stop_can_be_requested_before_terminal_stage() {
    assert!(OutputThreadWorkerLifecycleStage::ContractOnly.can_request_stop());
    assert!(OutputThreadWorkerLifecycleStage::HandleNotStarted.can_request_stop());
}

#[test]
fn stop_cannot_be_requested_after_terminal_stage() {
    assert!(!OutputThreadWorkerLifecycleStage::Stopped.can_request_stop());
    assert!(!OutputThreadWorkerLifecycleStage::Failed.can_request_stop());
}

#[test]
fn stage_names_do_not_imply_ready_or_running() {
    let stages = [
        OutputThreadWorkerLifecycleStage::NotCreated,
        OutputThreadWorkerLifecycleStage::ContractOnly,
        OutputThreadWorkerLifecycleStage::HandleNotStarted,
        OutputThreadWorkerLifecycleStage::StopRequested,
        OutputThreadWorkerLifecycleStage::Stopped,
        OutputThreadWorkerLifecycleStage::Failed,
    ];
    for s in stages {
        let name = format!("{:?}", s);
        assert!(!name.contains("Ready"), "{:?} contains Ready", s);
        assert!(!name.contains("Running"), "{:?} contains Running", s);
        assert!(!name.contains("Active"), "{:?} contains Active", s);
    }
}

#[test]
fn not_created_cannot_request_stop() {
    assert!(!OutputThreadWorkerLifecycleStage::NotCreated.can_request_stop());
}

#[test]
fn stop_requested_is_not_terminal() {
    assert!(!OutputThreadWorkerLifecycleStage::StopRequested.is_terminal());
}
