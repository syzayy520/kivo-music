use super::output_thread_worker_handle_contract::*;
use super::output_thread_worker_lifecycle::*;
use super::output_thread_worker_lifecycle_plan::*;
use super::output_thread_worker_shutdown::*;

#[test]
fn none_keeps_current_stage() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract::contract_only(),
        request: OutputThreadWorkerShutdownRequest::None,
    };
    let d = plan_worker_lifecycle(input);
    assert_eq!(d.next_stage, OutputThreadWorkerLifecycleStage::ContractOnly);
}

#[test]
fn stop_request_without_worker_returns_no_worker() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract::contract_only(),
        request: OutputThreadWorkerShutdownRequest::RequestStop,
    };
    let d = plan_worker_lifecycle(input);
    assert_eq!(d.outcome, OutputThreadWorkerShutdownOutcome::NoWorker);
}

#[test]
fn close_transport_marks_stopped() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract::contract_only(),
        request: OutputThreadWorkerShutdownRequest::CloseTransport,
    };
    let d = plan_worker_lifecycle(input);
    assert_eq!(d.next_stage, OutputThreadWorkerLifecycleStage::Stopped);
}

#[test]
fn close_transport_stops_scaffold_continuation() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract::contract_only(),
        request: OutputThreadWorkerShutdownRequest::CloseTransport,
    };
    let d = plan_worker_lifecycle(input);
    assert!(!d.should_continue_scaffold);
}

#[test]
fn terminal_stage_stop_request_returns_already_stopped() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract {
            lifecycle: OutputThreadWorkerLifecycleStage::Stopped,
            ..OutputThreadWorkerHandleContract::contract_only()
        },
        request: OutputThreadWorkerShutdownRequest::RequestStop,
    };
    let d = plan_worker_lifecycle(input);
    assert_eq!(d.outcome, OutputThreadWorkerShutdownOutcome::AlreadyStopped);
}

#[test]
fn plan_never_reports_live_worker() {
    let scenarios = [
        OutputThreadWorkerShutdownRequest::None,
        OutputThreadWorkerShutdownRequest::RequestStop,
        OutputThreadWorkerShutdownRequest::CloseTransport,
    ];
    for req in scenarios {
        let input = OutputThreadWorkerLifecycleInput {
            handle: OutputThreadWorkerHandleContract::contract_only(),
            request: req,
        };
        let d = plan_worker_lifecycle(input);
        assert!(!d.next_stage.has_live_worker());
    }
}

#[test]
fn plan_does_not_model_thread_start() {
    let input = OutputThreadWorkerLifecycleInput {
        handle: OutputThreadWorkerHandleContract::contract_only(),
        request: OutputThreadWorkerShutdownRequest::None,
    };
    let d = plan_worker_lifecycle(input);
    let name = format!("{:?}", d.next_stage);
    assert!(!name.contains("Running"));
    assert!(!name.contains("Active"));
}
