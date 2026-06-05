use super::super::output_thread_core::render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::scenario::{
    OutputThreadRuntimeLoopScenario, OutputThreadRuntimeLoopScenarioKind,
};
use super::state::OutputThreadRuntimeLoopState;
use super::super::runtime_queue_bridge::bridge::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeProjection,
    OutputThreadRuntimeQueueBridgeResult,
};
use super::super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;

fn default_projection() -> OutputThreadRuntimeQueueBridgeProjection {
    OutputThreadRuntimeQueueBridgeProjection {
        runtime_state: super::super::output_thread_core::state::OutputThreadState::Created,
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

fn accepted_start_result() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(
            OutputThreadRuntimeGeneration(0),
            OutputThreadRuntimeIntent::Start,
            0,
        ),
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: default_projection(),
    })
}

fn render_audio_plan() -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 100,
        ..OutputThreadRenderPlan::default()
    }
}

#[test]
fn scenario_new_preserves_fields() {
    let scenario = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::AcceptedStart,
        "test_scenario",
        OutputThreadRuntimeLoopState::Idle,
        vec![accepted_start_result()],
        vec![render_audio_plan()],
        5,
    );
    assert_eq!(scenario.kind, OutputThreadRuntimeLoopScenarioKind::AcceptedStart);
    assert_eq!(scenario.name, "test_scenario");
    assert_eq!(scenario.initial_state, OutputThreadRuntimeLoopState::Idle);
    assert_eq!(scenario.queue_results.len(), 1);
    assert_eq!(scenario.render_plans.len(), 1);
    assert_eq!(scenario.max_steps, 5);
}

#[test]
fn scenario_identifies_queue_results() {
    let with_results = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::AcceptedStart,
        "with_results",
        OutputThreadRuntimeLoopState::Idle,
        vec![accepted_start_result()],
        vec![],
        1,
    );
    assert!(with_results.has_queue_results());

    let without_results = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan,
        "without_results",
        OutputThreadRuntimeLoopState::Active,
        vec![],
        vec![render_audio_plan()],
        1,
    );
    assert!(!without_results.has_queue_results());
}

#[test]
fn scenario_identifies_render_plans() {
    let with_plans = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan,
        "with_plans",
        OutputThreadRuntimeLoopState::Active,
        vec![],
        vec![render_audio_plan()],
        1,
    );
    assert!(with_plans.has_render_plans());

    let without_plans = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::AcceptedStart,
        "without_plans",
        OutputThreadRuntimeLoopState::Idle,
        vec![accepted_start_result()],
        vec![],
        1,
    );
    assert!(!without_plans.has_render_plans());
}

#[test]
fn scenario_is_pure_scenario() {
    let scenario = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::RenderExitPlan,
        "pure_test",
        OutputThreadRuntimeLoopState::Active,
        vec![],
        vec![OutputThreadRenderPlan {
            action: OutputThreadRenderAction::Exit,
            ..OutputThreadRenderPlan::default()
        }],
        1,
    );
    assert!(scenario.is_pure_scenario());
}

#[test]
fn scenario_max_steps_is_a_pure_limit() {
    let scenario = OutputThreadRuntimeLoopScenario::new(
        OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan,
        "limit_test",
        OutputThreadRuntimeLoopState::Active,
        vec![],
        vec![render_audio_plan(); 10],
        3,
    );
    assert_eq!(scenario.max_steps, 3);
    assert_eq!(scenario.render_plans.len(), 10);
}
