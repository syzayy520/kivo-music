use super::output_thread_plan_projection::{project_stats_from_plan, OutputThreadStatsProjection};
use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};

#[test]
fn audio_plan_projects_consumed_and_rendered_frames() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: 256,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    };
    let proj = project_stats_from_plan(plan);
    assert_eq!(proj.consumed_frames_delta, 256);
    assert_eq!(proj.rendered_frames_delta, 256);
    assert_eq!(proj.silence_filled_frames_delta, 0);
    assert_eq!(proj.dropped_frames_delta, 0);
    assert!(!proj.should_sleep);
    assert!(!proj.should_exit);
}

#[test]
fn silence_plan_projects_rendered_and_silence_frames() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderSilence,
        frames_to_read: 0,
        silence_frames: 128,
        should_sleep: false,
        should_exit: false,
    };
    let proj = project_stats_from_plan(plan);
    assert_eq!(proj.consumed_frames_delta, 0);
    assert_eq!(proj.rendered_frames_delta, 128);
    assert_eq!(proj.silence_filled_frames_delta, 128);
    assert_eq!(proj.dropped_frames_delta, 0);
    assert!(!proj.should_sleep);
    assert!(!proj.should_exit);
}

#[test]
fn sleep_plan_projects_sleep_only() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Sleep,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: true,
        should_exit: false,
    };
    let proj = project_stats_from_plan(plan);
    assert_eq!(
        proj,
        OutputThreadStatsProjection {
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: true,
            should_exit: false,
        }
    );
}

#[test]
fn exit_plan_projects_exit_only() {
    let plan = OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: true,
    };
    let proj = project_stats_from_plan(plan);
    assert_eq!(
        proj,
        OutputThreadStatsProjection {
            consumed_frames_delta: 0,
            rendered_frames_delta: 0,
            silence_filled_frames_delta: 0,
            dropped_frames_delta: 0,
            should_sleep: false,
            should_exit: true,
        }
    );
}

#[test]
fn projection_never_projects_dropped_frames() {
    let plans = [
        OutputThreadRenderPlan {
            action: OutputThreadRenderAction::RenderAudio,
            frames_to_read: 64,
            ..Default::default()
        },
        OutputThreadRenderPlan {
            action: OutputThreadRenderAction::RenderSilence,
            silence_frames: 32,
            ..Default::default()
        },
        OutputThreadRenderPlan {
            action: OutputThreadRenderAction::Sleep,
            should_sleep: true,
            ..Default::default()
        },
        OutputThreadRenderPlan {
            action: OutputThreadRenderAction::Exit,
            should_exit: true,
            ..Default::default()
        },
    ];
    for plan in plans {
        let proj = project_stats_from_plan(plan);
        assert_eq!(proj.dropped_frames_delta, 0);
    }
}
