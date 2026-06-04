use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_plan::plan_runtime_loop_step;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_loop_step::{
    OutputThreadRuntimeLoopStepAction, OutputThreadRuntimeLoopStepDecision,
    OutputThreadRuntimeLoopStepInput,
};
use super::output_thread_runtime_queue_bridge::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeProjection,
    OutputThreadRuntimeQueueBridgeRejected, OutputThreadRuntimeQueueBridgeResult,
};
use super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::runtime_queue::state::OutputThreadRuntimeQueueState;
fn default_projection() -> OutputThreadRuntimeQueueBridgeProjection {
    OutputThreadRuntimeQueueBridgeProjection {
        runtime_state: super::output_thread_state::OutputThreadState::Created,
        pending_count: 0,
        accepted_count: 0,
        rejected_count: 0,
        last_sequence: 0,
        queue_closed: false,
        queue_has_capacity: true,
        runtime_can_accept_frames: false,
        bridge_can_accept_intents: true,
    }
}
fn default_queue_state() -> OutputThreadRuntimeQueueState {
    OutputThreadRuntimeQueueState {
        pending_count: 0,
        accepted_count: 0,
        rejected_count: 0,
        last_sequence: 0,
        closed: false,
    }
}
fn accepted_result(intent: OutputThreadRuntimeIntent) -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(OutputThreadRuntimeGeneration(0), intent, 0),
        queue_state: default_queue_state(),
        projection: default_projection(),
    })
}
fn rejected_result() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Rejected(OutputThreadRuntimeQueueBridgeRejected {
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: default_queue_state(),
        projection: default_projection(),
    })
}
fn plan_with(state: OutputThreadRuntimeLoopState, result: Option<OutputThreadRuntimeQueueBridgeResult>, action: OutputThreadRenderAction) -> OutputThreadRuntimeLoopStepDecision {
    plan_runtime_loop_step(OutputThreadRuntimeLoopStepInput::new(state, result, OutputThreadRenderPlan { action, ..OutputThreadRenderPlan::default() }))
}
fn plan_no_queue(state: OutputThreadRuntimeLoopState, action: OutputThreadRenderAction) -> OutputThreadRuntimeLoopStepDecision {
    plan_runtime_loop_step(OutputThreadRuntimeLoopStepInput::without_queue_result(state, OutputThreadRenderPlan { action, ..OutputThreadRenderPlan::default() }))
}
#[test]
fn idle_start_accepted_enters_active() {
    let d = plan_with(OutputThreadRuntimeLoopState::Idle, Some(accepted_result(OutputThreadRuntimeIntent::Start)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::EnterActive);
    assert!(d.continues());
}
#[test]
fn active_stop_accepted_marks_stopping() {
    let d = plan_with(OutputThreadRuntimeLoopState::Active, Some(accepted_result(OutputThreadRuntimeIntent::Stop)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Stopping);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::MarkStopping);
    assert!(d.continues());
}
#[test]
fn close_accepted_marks_exited() {
    let d = plan_with(OutputThreadRuntimeLoopState::Active, Some(accepted_result(OutputThreadRuntimeIntent::Close)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Exited);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::MarkExited);
    assert!(d.stops());
}
#[test]
fn pause_accepted_follows_render() {
    let d = plan_with(OutputThreadRuntimeLoopState::Active, Some(accepted_result(OutputThreadRuntimeIntent::Pause)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn rejected_intent_ignored() {
    let d = plan_with(OutputThreadRuntimeLoopState::Active, Some(rejected_result()), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent);
    assert!(d.continues());
}
#[test]
fn idle_sleep_stays_idle() {
    let d = plan_no_queue(OutputThreadRuntimeLoopState::Idle, OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Idle);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::StayIdle);
    assert!(d.continues());
}
#[test]
fn active_sleep_follows_render() {
    let d = plan_no_queue(OutputThreadRuntimeLoopState::Active, OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn exit_marks_exited() {
    let d = plan_no_queue(OutputThreadRuntimeLoopState::Active, OutputThreadRenderAction::Exit);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Exited);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::MarkExited);
    assert!(d.stops());
}
#[test]
fn render_audio_follows_render() {
    let d = plan_no_queue(OutputThreadRuntimeLoopState::Active, OutputThreadRenderAction::RenderAudio);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn render_silence_follows_render() {
    let d = plan_no_queue(OutputThreadRuntimeLoopState::Active, OutputThreadRenderAction::RenderSilence);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn active_start_accepted_follows_render() {
    let d = plan_with(OutputThreadRuntimeLoopState::Active, Some(accepted_result(OutputThreadRuntimeIntent::Start)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Active);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn idle_stop_accepted_follows_render() {
    let d = plan_with(OutputThreadRuntimeLoopState::Idle, Some(accepted_result(OutputThreadRuntimeIntent::Stop)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Idle);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::FollowRenderPlan);
    assert!(d.continues());
}
#[test]
fn stopped_close_accepted_marks_exited() {
    let d = plan_with(OutputThreadRuntimeLoopState::Stopped, Some(accepted_result(OutputThreadRuntimeIntent::Close)), OutputThreadRenderAction::Sleep);
    assert_eq!(d.next_state, OutputThreadRuntimeLoopState::Exited);
    assert_eq!(d.action, OutputThreadRuntimeLoopStepAction::MarkExited);
    assert!(d.stops());
}
