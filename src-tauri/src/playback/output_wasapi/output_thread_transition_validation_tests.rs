use super::output_thread_buffer_snapshot::OutputThreadBufferSnapshot;
use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_plan_validation::OutputThreadPlanValidationError;
use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::output_thread_state::OutputThreadState;
use super::output_thread_transition_validation::validate_plan_for_state;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

fn open_buffer(frames: u32) -> OutputThreadBufferSnapshot {
    OutputThreadBufferSnapshot::new(frames, false)
}

fn audio_plan(frames: u32) -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderAudio,
        frames_to_read: frames,
        silence_frames: 0,
        should_sleep: false,
        should_exit: false,
    }
}

fn silence_plan(frames: u32) -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::RenderSilence,
        frames_to_read: 0,
        silence_frames: frames,
        should_sleep: false,
        should_exit: false,
    }
}

fn sleep_plan() -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Sleep,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: true,
        should_exit: false,
    }
}

fn exit_plan() -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        frames_to_read: 0,
        silence_frames: 0,
        should_sleep: false,
        should_exit: true,
    }
}

#[test]
fn shutdown_requires_exit_plan() {
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let buffer = open_buffer(100);
    let plan = sleep_plan();
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::ShutdownPlanMustExit)
    );
}

#[test]
fn closed_empty_requires_exit_plan() {
    let buffer = OutputThreadBufferSnapshot::new(0, true);
    let control = running_control();
    let plan = sleep_plan();
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::ClosedExhaustedPlanMustExit)
    );
}

#[test]
fn non_running_requires_sleep_plan() {
    let control = OutputThreadControlSnapshot {
        state: OutputThreadState::Created,
        ..running_control()
    };
    let buffer = open_buffer(100);
    let plan = audio_plan(50);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::NonRunningPlanMustSleep)
    );
}

#[test]
fn paused_rejects_audio_plan() {
    let control = OutputThreadControlSnapshot {
        paused: true,
        ..running_control()
    };
    let buffer = open_buffer(100);
    let plan = audio_plan(50);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::PausedPlanMustNotRenderAudio)
    );
}

#[test]
fn flush_rejects_audio_plan() {
    let control = OutputThreadControlSnapshot {
        flush_requested: true,
        ..running_control()
    };
    let buffer = open_buffer(100);
    let plan = audio_plan(50);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::FlushPlanMustNotRenderAudio)
    );
}

#[test]
fn empty_running_buffer_rejects_audio_plan() {
    let control = running_control();
    let buffer = open_buffer(0);
    let plan = audio_plan(1);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames)
    );
}

#[test]
fn audio_plan_cannot_read_more_than_available() {
    let control = running_control();
    let buffer = open_buffer(50);
    let plan = audio_plan(100);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::AudioPlanExceedsAvailable)
    );
}

#[test]
fn audio_plan_with_available_frames_passes() {
    let control = running_control();
    let buffer = open_buffer(256);
    let plan = audio_plan(128);
    assert!(validate_plan_for_state(buffer, control, plan).is_ok());
}

#[test]
fn silence_plan_for_paused_state_passes() {
    let control = OutputThreadControlSnapshot {
        paused: true,
        ..running_control()
    };
    let buffer = open_buffer(0);
    let plan = silence_plan(64);
    assert!(validate_plan_for_state(buffer, control, plan).is_ok());
}

#[test]
fn exit_plan_for_shutdown_passes() {
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let buffer = open_buffer(100);
    let plan = exit_plan();
    assert!(validate_plan_for_state(buffer, control, plan).is_ok());
}

#[test]
fn closed_buffer_with_remaining_frames_allows_audio() {
    let buffer = OutputThreadBufferSnapshot::new(64, true);
    let control = running_control();
    let plan = audio_plan(32);
    assert!(validate_plan_for_state(buffer, control, plan).is_ok());
}

#[test]
fn shutdown_has_priority_over_closed_remaining_frames() {
    let buffer = OutputThreadBufferSnapshot::new(64, true);
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let plan = audio_plan(32);
    assert_eq!(
        validate_plan_for_state(buffer, control, plan),
        Err(OutputThreadPlanValidationError::ShutdownPlanMustExit)
    );
}
