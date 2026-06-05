use super::super::output_thread_core::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::scenario::{
    OutputThreadRuntimeLoopScenario, OutputThreadRuntimeLoopScenarioKind,
};
use super::scenario_runner::plan_runtime_loop_scenario;
use super::state::OutputThreadRuntimeLoopState;
use super::step::OutputThreadRuntimeLoopStepAction;
use super::super::runtime_queue_bridge::bridge::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeProjection,
    OutputThreadRuntimeQueueBridgeRejected, OutputThreadRuntimeQueueBridgeResult,
};
use super::super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
fn default_projection() -> OutputThreadRuntimeQueueBridgeProjection {
    OutputThreadRuntimeQueueBridgeProjection {
        runtime_state: super::super::output_thread_core::state::OutputThreadState::Created,
        pending_count: 0, accepted_count: 0, rejected_count: 0, last_sequence: 0,
        queue_closed: false, queue_has_capacity: true,
        runtime_can_accept_frames: false, bridge_can_accept_intents: true,
    }
}
fn accepted(intent: OutputThreadRuntimeIntent) -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(OutputThreadRuntimeGeneration(0), intent, 0),
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: default_projection(),
    })
}
fn rejected() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Rejected(OutputThreadRuntimeQueueBridgeRejected {
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: default_projection(),
    })
}
fn scenario(kind: OutputThreadRuntimeLoopScenarioKind, name: &'static str, initial: OutputThreadRuntimeLoopState, queue: Vec<OutputThreadRuntimeQueueBridgeResult>, plans: Vec<OutputThreadRenderPlan>) -> OutputThreadRuntimeLoopScenario {
    OutputThreadRuntimeLoopScenario::new(kind, name, initial, queue, plans, 10)
}
#[test]
fn accepted_start_scenario_enters_active() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::AcceptedStart, "start", OutputThreadRuntimeLoopState::Idle, vec![accepted(OutputThreadRuntimeIntent::Start)], vec![OutputThreadRenderPlan::default()]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::EnterActive);
}
#[test]
fn rejected_start_scenario_keeps_initial_state() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RejectedStart, "rej", OutputThreadRuntimeLoopState::Idle, vec![rejected()], vec![OutputThreadRenderPlan::default()]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Idle);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent);
}
#[test]
fn accepted_stop_scenario_marks_stopping() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::AcceptedStop, "stop", OutputThreadRuntimeLoopState::Active, vec![accepted(OutputThreadRuntimeIntent::Stop)], vec![OutputThreadRenderPlan::default()]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Stopping);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::MarkStopping);
}
#[test]
fn accepted_close_scenario_exits_and_stops() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::AcceptedClose, "close", OutputThreadRuntimeLoopState::Active, vec![accepted(OutputThreadRuntimeIntent::Close)], vec![OutputThreadRenderPlan::default()]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Exited);
    assert!(r.stopped_by_decision);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::MarkExited);
}
#[test]
fn render_exit_plan_marks_exited() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RenderExitPlan, "exit", OutputThreadRuntimeLoopState::Active, vec![], vec![OutputThreadRenderPlan { action: OutputThreadRenderAction::Exit, ..OutputThreadRenderPlan::default() }]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Exited);
    assert!(r.stopped_by_decision);
}
#[test]
fn render_audio_plan_only_follows_plan() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan, "audio", OutputThreadRuntimeLoopState::Active, vec![], vec![OutputThreadRenderPlan { action: OutputThreadRenderAction::RenderAudio, frames_to_read: 100, ..OutputThreadRenderPlan::default() }]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
}
#[test]
fn render_silence_plan_only_follows_plan() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RenderSilencePlan, "silence", OutputThreadRuntimeLoopState::Active, vec![], vec![OutputThreadRenderPlan { action: OutputThreadRenderAction::RenderSilence, silence_frames: 50, ..OutputThreadRenderPlan::default() }]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(r.decisions[0].action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
}
#[test]
fn runner_records_pure_decisions() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan, "multi", OutputThreadRuntimeLoopState::Active, vec![], vec![OutputThreadRenderPlan { action: OutputThreadRenderAction::RenderAudio, ..OutputThreadRenderPlan::default() }, OutputThreadRenderPlan { action: OutputThreadRenderAction::RenderAudio, ..OutputThreadRenderPlan::default() }]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.decision_count(), 2);
    assert!(r.has_decisions());
}
#[test]
fn runner_does_not_consume_production_queue() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::AcceptedStart, "noq", OutputThreadRuntimeLoopState::Idle, vec![accepted(OutputThreadRuntimeIntent::Start)], vec![OutputThreadRenderPlan::default()]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.decision_count(), 1);
    assert!(r.has_decisions());
}
#[test]
fn runner_does_not_model_audible_output() {
    let s = scenario(OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan, "class", OutputThreadRuntimeLoopState::Active, vec![], vec![OutputThreadRenderPlan { action: OutputThreadRenderAction::RenderAudio, frames_to_read: 1000, ..OutputThreadRenderPlan::default() }]);
    let r = plan_runtime_loop_scenario(&s);
    assert_eq!(r.final_state, OutputThreadRuntimeLoopState::Active);
    assert!(!r.is_terminal());
}
