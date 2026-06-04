use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_scenario::{
    OutputThreadRuntimeLoopScenario, OutputThreadRuntimeLoopScenarioKind,
};
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::runtime_queue_bridge::bridge::{
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

fn accepted_result(intent: OutputThreadRuntimeIntent) -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(OutputThreadRuntimeGeneration(0), intent, 0),
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: default_projection(),
    })
}

fn rejected_result() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Rejected(OutputThreadRuntimeQueueBridgeRejected {
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: default_projection(),
    })
}

fn render_plan(action: OutputThreadRenderAction) -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action,
        ..OutputThreadRenderPlan::default()
    }
}

/// Returns the fixed mock runtime loop scenario matrix.
#[allow(dead_code)]
pub(crate) fn runtime_loop_scenario_matrix() -> Vec<OutputThreadRuntimeLoopScenario> {
    vec![
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::AcceptedStart,
            "accepted_start",
            OutputThreadRuntimeLoopState::Idle,
            vec![accepted_result(OutputThreadRuntimeIntent::Start)],
            vec![OutputThreadRenderPlan::default()],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::RejectedStart,
            "rejected_start",
            OutputThreadRuntimeLoopState::Idle,
            vec![rejected_result()],
            vec![OutputThreadRenderPlan::default()],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan,
            "active_render_audio_plan",
            OutputThreadRuntimeLoopState::Active,
            vec![],
            vec![render_plan(OutputThreadRenderAction::RenderAudio)],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::RenderSilencePlan,
            "active_render_silence_plan",
            OutputThreadRuntimeLoopState::Active,
            vec![],
            vec![render_plan(OutputThreadRenderAction::RenderSilence)],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::AcceptedStop,
            "accepted_stop",
            OutputThreadRuntimeLoopState::Active,
            vec![accepted_result(OutputThreadRuntimeIntent::Stop)],
            vec![OutputThreadRenderPlan::default()],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::AcceptedClose,
            "accepted_close",
            OutputThreadRuntimeLoopState::Active,
            vec![accepted_result(OutputThreadRuntimeIntent::Close)],
            vec![OutputThreadRenderPlan::default()],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::RenderExitPlan,
            "render_exit_plan",
            OutputThreadRuntimeLoopState::Active,
            vec![],
            vec![render_plan(OutputThreadRenderAction::Exit)],
            1,
        ),
        OutputThreadRuntimeLoopScenario::new(
            OutputThreadRuntimeLoopScenarioKind::RejectedIntentIgnored,
            "rejected_intent_ignored",
            OutputThreadRuntimeLoopState::Active,
            vec![rejected_result()],
            vec![OutputThreadRenderPlan::default()],
            1,
        ),
    ]
}
