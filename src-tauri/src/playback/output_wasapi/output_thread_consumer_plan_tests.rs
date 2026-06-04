use super::output_thread_buffer_snapshot::OutputThreadBufferSnapshot;
use super::output_thread_consumer_plan::plan_consumer_step;
use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_render_plan::OutputThreadRenderAction;
use super::output_thread_state::OutputThreadState;

// ---------------------------------------------------------------------------
// Priority rules
// ---------------------------------------------------------------------------

#[test]
fn shutdown_requested_exits() {
    let buffer = OutputThreadBufferSnapshot::new(512, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: true,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Exit);
    assert!(plan.should_exit);
}

#[test]
fn closed_empty_buffer_exits() {
    let buffer = OutputThreadBufferSnapshot::new(0, true);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Exit);
    assert!(plan.should_exit);
}

#[test]
fn non_running_state_sleeps() {
    let buffer = OutputThreadBufferSnapshot::new(512, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Created,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Sleep);
    assert!(plan.should_sleep);
}

#[test]
fn paused_with_free_frames_renders_silence() {
    let buffer = OutputThreadBufferSnapshot::new(0, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: true,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 128, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderSilence);
    assert_eq!(plan.silence_frames, 128);
}

#[test]
fn paused_without_free_frames_sleeps() {
    let buffer = OutputThreadBufferSnapshot::new(0, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: true,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 0, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Sleep);
}

#[test]
fn flush_with_free_frames_renders_silence() {
    let buffer = OutputThreadBufferSnapshot::new(0, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: true,
    };
    let plan = plan_consumer_step(buffer, 200, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderSilence);
    assert_eq!(plan.silence_frames, 200);
}

#[test]
fn flush_without_free_frames_sleeps() {
    let buffer = OutputThreadBufferSnapshot::new(0, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: true,
    };
    let plan = plan_consumer_step(buffer, 0, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Sleep);
}

#[test]
fn no_free_frames_sleeps() {
    let buffer = OutputThreadBufferSnapshot::new(512, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 0, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Sleep);
}

#[test]
fn available_frames_render_audio_limited_by_free_frames() {
    let buffer = OutputThreadBufferSnapshot::new(1024, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderAudio);
    assert_eq!(plan.frames_to_read, 256);
}

#[test]
fn available_frames_render_audio_limited_by_available_frames() {
    let buffer = OutputThreadBufferSnapshot::new(64, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderAudio);
    assert_eq!(plan.frames_to_read, 64);
}

#[test]
fn empty_open_buffer_renders_silence() {
    let buffer = OutputThreadBufferSnapshot::new(0, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderSilence);
    assert_eq!(plan.silence_frames, 256);
}

#[test]
fn closed_buffer_with_remaining_frames_renders_audio_before_exit() {
    let buffer = OutputThreadBufferSnapshot::new(128, true);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::RenderAudio);
    assert_eq!(plan.frames_to_read, 128);
}

#[test]
fn shutdown_has_priority_over_available_frames() {
    let buffer = OutputThreadBufferSnapshot::new(1024, false);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: true,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Exit);
}

#[test]
fn closed_empty_has_priority_over_non_running() {
    let buffer = OutputThreadBufferSnapshot::new(0, true);
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Created,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    let plan = plan_consumer_step(buffer, 256, control);
    assert_eq!(plan.action, OutputThreadRenderAction::Exit);
}
