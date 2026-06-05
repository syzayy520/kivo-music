use super::super::output_thread_core::render_plan::OutputThreadRenderPlan;
use super::state::OutputThreadRuntimeLoopState;
use super::step::{
    OutputThreadRuntimeLoopStepAction, OutputThreadRuntimeLoopStepDecision,
    OutputThreadRuntimeLoopStepInput,
};

#[test]
fn step_input_new_with_queue_result() {
    let input = OutputThreadRuntimeLoopStepInput::new(
        OutputThreadRuntimeLoopState::Idle,
        None,
        OutputThreadRenderPlan::default(),
    );
    assert_eq!(input.state, OutputThreadRuntimeLoopState::Idle);
    assert!(input.queue_result.is_none());
}

#[test]
fn step_input_without_queue_result() {
    let input = OutputThreadRuntimeLoopStepInput::without_queue_result(
        OutputThreadRuntimeLoopState::Active,
        OutputThreadRenderPlan::default(),
    );
    assert_eq!(input.state, OutputThreadRuntimeLoopState::Active);
    assert!(input.queue_result.is_none());
}

#[test]
fn step_decision_continues_when_should_continue() {
    let decision = OutputThreadRuntimeLoopStepDecision::new(
        OutputThreadRuntimeLoopState::Active,
        OutputThreadRuntimeLoopStepAction::EnterActive,
        true,
    );
    assert!(decision.continues());
    assert!(!decision.stops());
}

#[test]
fn step_decision_stops_when_not_should_continue() {
    let decision = OutputThreadRuntimeLoopStepDecision::new(
        OutputThreadRuntimeLoopState::Exited,
        OutputThreadRuntimeLoopStepAction::MarkExited,
        false,
    );
    assert!(!decision.continues());
    assert!(decision.stops());
}

#[test]
fn step_action_variants_are_distinct() {
    let actions = [
        OutputThreadRuntimeLoopStepAction::StayIdle,
        OutputThreadRuntimeLoopStepAction::EnterActive,
        OutputThreadRuntimeLoopStepAction::MarkStopping,
        OutputThreadRuntimeLoopStepAction::MarkStopped,
        OutputThreadRuntimeLoopStepAction::MarkExited,
        OutputThreadRuntimeLoopStepAction::FollowRenderPlan,
        OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent,
    ];
    for (i, a) in actions.iter().enumerate() {
        for (j, b) in actions.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn step_input_equality() {
    let a = OutputThreadRuntimeLoopStepInput::without_queue_result(
        OutputThreadRuntimeLoopState::Idle,
        OutputThreadRenderPlan::default(),
    );
    let b = OutputThreadRuntimeLoopStepInput::without_queue_result(
        OutputThreadRuntimeLoopState::Idle,
        OutputThreadRenderPlan::default(),
    );
    assert_eq!(a, b);
}

#[test]
fn step_decision_equality() {
    let a = OutputThreadRuntimeLoopStepDecision::new(
        OutputThreadRuntimeLoopState::Active,
        OutputThreadRuntimeLoopStepAction::EnterActive,
        true,
    );
    let b = OutputThreadRuntimeLoopStepDecision::new(
        OutputThreadRuntimeLoopState::Active,
        OutputThreadRuntimeLoopStepAction::EnterActive,
        true,
    );
    assert_eq!(a, b);
}
