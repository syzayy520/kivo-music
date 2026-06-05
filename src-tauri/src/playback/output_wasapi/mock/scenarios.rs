use super::super::output_thread_core::control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::renderer::OutputThreadMockRenderer;
use super::scenario::OutputThreadMockScenario;
use super::super::output_thread_core::state::OutputThreadState;

// ---------------------------------------------------------------------------
// Control helpers
// ---------------------------------------------------------------------------

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

fn paused_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        paused: true,
        ..running_control()
    }
}

fn shutdown_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    }
}

fn flush_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        flush_requested: true,
        ..running_control()
    }
}

fn non_running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Created,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

// ---------------------------------------------------------------------------
// Named scenario factories
// ---------------------------------------------------------------------------

/// Normal audio playback: buffer has frames, renderer has capacity.
#[allow(dead_code)]
pub(crate) fn normal_audio() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "normal_audio",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    )
}

/// Buffer empty, renderer has capacity → silence.
#[allow(dead_code)]
pub(crate) fn empty_running() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "empty_running",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        2,
    )
}

/// Renderer full → sleep.
#[allow(dead_code)]
pub(crate) fn no_capacity() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "no_capacity",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::full(),
        running_control(),
        2,
    )
}

/// Shutdown requested → exit immediately.
#[allow(dead_code)]
pub(crate) fn shutdown_requested() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "shutdown_requested",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        shutdown_control(),
        2,
    )
}

/// Paused, buffer empty → silence or sleep.
#[allow(dead_code)]
pub(crate) fn paused_empty() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "paused_empty",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::with_free_frames(50),
        paused_control(),
        2,
    )
}

/// Paused, buffer has frames → silence (not audio).
#[allow(dead_code)]
pub(crate) fn paused_with_frames() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "paused_with_frames",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        paused_control(),
        2,
    )
}

/// Flush requested, buffer empty → silence or sleep.
#[allow(dead_code)]
pub(crate) fn flush_empty() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "flush_empty",
        OutputThreadMockBuffer::empty_open(),
        OutputThreadMockRenderer::with_free_frames(50),
        flush_control(),
        2,
    )
}

/// Buffer closed and empty → exit.
#[allow(dead_code)]
pub(crate) fn closed_empty() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "closed_empty",
        OutputThreadMockBuffer::closed_empty(),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        2,
    )
}

/// Buffer closed with remaining frames → drain then exit.
#[allow(dead_code)]
pub(crate) fn closed_with_remaining() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "closed_with_remaining",
        OutputThreadMockBuffer::closed_with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        running_control(),
        4,
    )
}

/// State not running → sleep.
#[allow(dead_code)]
pub(crate) fn non_running() -> OutputThreadMockScenario {
    OutputThreadMockScenario::new(
        "non_running",
        OutputThreadMockBuffer::with_frames(100),
        OutputThreadMockRenderer::with_free_frames(50),
        non_running_control(),
        2,
    )
}
