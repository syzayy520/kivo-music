use super::output_thread_mock_expectation::OutputThreadMockExpectation;
use super::output_thread_render_plan::OutputThreadRenderAction;

/// Normal audio scenario: buffer has frames, renderer has capacity.
#[allow(dead_code)]
pub(crate) fn normal_audio_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        10,
        100,
        0,
        false,
        false,
        true,
    )
}

/// Empty running scenario: buffer empty, renderer full.
#[allow(dead_code)]
pub(crate) fn empty_running_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        10,
        0,
        50,
        false,
        false,
        true,
    )
}

/// No capacity scenario: buffer has frames, renderer full.
#[allow(dead_code)]
pub(crate) fn no_capacity_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Sleep,
        1,
        5,
        0,
        0,
        false,
        true,
        true,
    )
}

/// Shutdown requested scenario: control requests shutdown.
#[allow(dead_code)]
pub(crate) fn shutdown_requested_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Exit,
        1,
        2,
        0,
        0,
        true,
        false,
        true,
    )
}

/// Paused empty scenario: paused, buffer empty.
#[allow(dead_code)]
pub(crate) fn paused_empty_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        10,
        0,
        50,
        false,
        false,
        true,
    )
}

/// Paused with frames scenario: paused, buffer has frames.
#[allow(dead_code)]
pub(crate) fn paused_with_frames_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        10,
        0,
        50,
        false,
        false,
        true,
    )
}

/// Flush empty scenario: flush requested, buffer empty.
#[allow(dead_code)]
pub(crate) fn flush_empty_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        10,
        0,
        50,
        false,
        false,
        true,
    )
}

/// Closed empty scenario: buffer closed, no frames.
#[allow(dead_code)]
pub(crate) fn closed_empty_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Exit,
        1,
        2,
        0,
        0,
        true,
        false,
        true,
    )
}

/// Closed with remaining scenario: buffer closed, has frames.
///
/// Audio frames are consumed, then buffer becomes closed+empty → exits.
#[allow(dead_code)]
pub(crate) fn closed_with_remaining_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Exit,
        3,
        4,
        100,
        0,
        true,
        false,
        true,
    )
}

/// Non running scenario: state not Running.
#[allow(dead_code)]
pub(crate) fn non_running_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Sleep,
        1,
        5,
        0,
        0,
        false,
        true,
        true,
    )
}

/// Look up expectation by scenario name.
#[allow(dead_code)]
pub(crate) fn expectation_for_name(name: &str) -> Option<OutputThreadMockExpectation> {
    match name {
        "normal_audio" => Some(normal_audio_expectation()),
        "empty_running" => Some(empty_running_expectation()),
        "no_capacity" => Some(no_capacity_expectation()),
        "shutdown_requested" => Some(shutdown_requested_expectation()),
        "paused_empty" => Some(paused_empty_expectation()),
        "paused_with_frames" => Some(paused_with_frames_expectation()),
        "flush_empty" => Some(flush_empty_expectation()),
        "closed_empty" => Some(closed_empty_expectation()),
        "closed_with_remaining" => Some(closed_with_remaining_expectation()),
        "non_running" => Some(non_running_expectation()),
        _ => None,
    }
}
