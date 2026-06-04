use super::output_thread_buffer_snapshot::OutputThreadBufferSnapshot;
use super::output_thread_control::OutputThreadControlSnapshot;
use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::output_thread_state::OutputThreadState;

/// Compute the next consumer step based on current state.
///
/// Pure function — no mutations, no external state, no buffer reads.
/// Rules are evaluated in strict priority order.
#[allow(dead_code)]
pub(crate) fn plan_consumer_step(
    buffer: OutputThreadBufferSnapshot,
    free_frames: u32,
    control: OutputThreadControlSnapshot,
) -> OutputThreadRenderPlan {
    // Rule 1: Shutdown requested → exit
    if control.shutdown_requested {
        return exit_plan();
    }

    // Rule 2: Buffer closed and exhausted → exit
    if buffer.is_closed && buffer.available_frames == 0 {
        return exit_plan();
    }

    // Rule 3: Not running → sleep
    if control.state != OutputThreadState::Running {
        return sleep_plan();
    }

    // Rule 4: Paused
    if control.paused {
        if free_frames > 0 {
            return silence_plan(free_frames);
        }
        return sleep_plan();
    }

    // Rule 5: Flush requested
    if control.flush_requested {
        if free_frames > 0 {
            return silence_plan(free_frames);
        }
        return sleep_plan();
    }

    // Rule 6: No free frames in render buffer → sleep
    if free_frames == 0 {
        return sleep_plan();
    }

    // Rule 7: Buffer has frames → render audio
    if buffer.available_frames > 0 {
        let frames = buffer.available_frames.min(free_frames);
        return audio_plan(frames);
    }

    // Rule 8: Buffer empty but open → render silence
    silence_plan(free_frames)
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

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
