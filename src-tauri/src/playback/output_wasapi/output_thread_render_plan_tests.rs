use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

#[test]
fn default_plan_is_sleep() {
    let plan = OutputThreadRenderPlan::default();
    assert_eq!(plan.action, OutputThreadRenderAction::Sleep);
    assert_eq!(plan.frames_to_read, 0);
    assert_eq!(plan.silence_frames, 0);
    assert!(!plan.should_sleep);
    assert!(!plan.should_exit);
}

#[test]
fn sleep_plan_shape() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Sleep,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: true,
        should_exit: false,
    };
    assert!(plan.is_idle());
    assert!(!plan.is_audio());
    assert!(!plan.is_silence());
    assert!(!plan.is_exit());
}

#[test]
fn audio_plan_shape() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 256,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    };
    assert!(plan.is_audio());
    assert!(!plan.is_idle());
    assert!(!plan.is_silence());
    assert!(!plan.is_exit());
}

#[test]
fn silence_plan_shape() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderSilence,
        frames_to_read: 0,
        silence_frames: 128,
        should_sleep: false,
        should_exit: false,
    };
    assert!(plan.is_silence());
    assert!(!plan.is_audio());
    assert!(!plan.is_idle());
    assert!(!plan.is_exit());
}

#[test]
fn exit_plan_shape() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: true,
    };
    assert!(plan.is_exit());
    assert!(!plan.is_audio());
    assert!(!plan.is_silence());
    assert!(!plan.is_idle());
}

#[test]
fn render_action_default_is_sleep() {
    let action = OutputThreadRenderAction::default();
    assert_eq!(action, OutputThreadRenderAction::Sleep);
}
