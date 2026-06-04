use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_worker_loop_report::OutputThreadWorkerLoopReport;
use super::output_thread_worker_loop_state::OutputThreadWorkerLoopState;
use super::output_thread_worker_loop_step::{
    OutputThreadWorkerLoopStepDecision, OutputThreadWorkerLoopStepKind,
};
use super::output_thread_worker_runtime_adapter::adapt_worker_step_to_runtime_loop;
use super::output_thread_worker_runtime_decision::{
    OutputThreadWorkerRuntimeDecision, OutputThreadWorkerRuntimeDecisionKind,
};
use super::output_thread_worker_runtime_report::OutputThreadWorkerRuntimeReport;
use super::output_thread_worker_shutdown::OutputThreadWorkerShutdownRequest;

fn make_adapter_result(
    kind: OutputThreadWorkerRuntimeDecisionKind,
    intent: Option<OutputThreadRuntimeIntent>,
) -> super::output_thread_worker_runtime_adapter::OutputThreadWorkerRuntimeAdapterResult {
    let wr_decision = OutputThreadWorkerRuntimeDecision {
        kind,
        worker_kind: OutputThreadWorkerLoopStepKind::NoCommand,
        observed_runtime_intent: intent,
        queue_bridge_used: false,
        has_output_behavior: false,
        should_continue_scaffold: true,
    };
    let worker_decision = OutputThreadWorkerLoopStepDecision {
        kind: OutputThreadWorkerLoopStepKind::NoCommand,
        next_state: OutputThreadWorkerLoopState::Polling,
        shutdown_request: OutputThreadWorkerShutdownRequest::None,
        should_continue: true,
    };
    let result = adapt_worker_step_to_runtime_loop(
        worker_decision,
        intent,
        OutputThreadRuntimeLoopState::Idle,
    );
    // Override the worker_runtime_decision with our test-specific one.
    super::output_thread_worker_runtime_adapter::OutputThreadWorkerRuntimeAdapterResult {
        worker_runtime_decision: wr_decision,
        runtime_loop_decision: result.runtime_loop_decision,
        queue_bridge_used: false,
        has_output_behavior: false,
    }
}

#[test]
fn empty_report_uses_no_queue_bridge() {
    let report = OutputThreadWorkerRuntimeReport::empty();
    assert!(!report.uses_queue_bridge());
}

#[test]
fn empty_report_has_no_output_behavior() {
    let report = OutputThreadWorkerRuntimeReport::empty();
    assert!(report.has_no_output_behavior());
}

#[test]
fn record_runtime_intent_increments_runtime_intents_observed() {
    let report = OutputThreadWorkerRuntimeReport::empty();
    let result = make_adapter_result(
        OutputThreadWorkerRuntimeDecisionKind::RuntimeIntentObserved,
        Some(OutputThreadRuntimeIntent::Start),
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.runtime_intents_observed, 1);
    assert_eq!(updated.shutdown_intents_observed, 0);
}

#[test]
fn record_shutdown_intent_increments_shutdown_intents_observed() {
    let report = OutputThreadWorkerRuntimeReport::empty();
    let result = make_adapter_result(
        OutputThreadWorkerRuntimeDecisionKind::RuntimeShutdownObserved,
        Some(OutputThreadRuntimeIntent::Stop),
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.shutdown_intents_observed, 1);
    assert_eq!(updated.runtime_intents_observed, 0);
}

#[test]
fn record_result_keeps_queue_bridge_false() {
    let report = OutputThreadWorkerRuntimeReport::empty();
    let result = make_adapter_result(
        OutputThreadWorkerRuntimeDecisionKind::NoRuntimeIntent,
        None,
    );
    let updated = report.record_adapter_result(result);
    assert!(!updated.uses_queue_bridge());
    assert!(updated.has_no_output_behavior());
}

#[test]
fn from_worker_report_copies_worker_step_count() {
    let worker_report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated_worker = worker_report
        .record_step(OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::NoCommand,
            next_state: OutputThreadWorkerLoopState::NoCommand,
            shutdown_request: OutputThreadWorkerShutdownRequest::None,
            should_continue: true,
        })
        .record_step(OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
            next_state: OutputThreadWorkerLoopState::CommandHandled,
            shutdown_request: OutputThreadWorkerShutdownRequest::None,
            should_continue: true,
        });

    let report = OutputThreadWorkerRuntimeReport::from_worker_report(updated_worker);
    assert_eq!(report.worker_steps, 2);
    assert_eq!(report.runtime_decisions, 0);
}
