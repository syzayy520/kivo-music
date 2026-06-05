use super::super::output_thread_control::OutputThreadControlSnapshot;
use super::buffer::OutputThreadMockBuffer;
use super::renderer::OutputThreadMockRenderer;
use super::sequence::run_mock_sequence;
use super::super::output_thread_render_plan::OutputThreadRenderAction;
use super::super::output_thread_state::OutputThreadState;

fn running_control() -> OutputThreadControlSnapshot {
    OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    }
}

#[test]
fn zero_steps_returns_empty_sequence() {
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let renderer = OutputThreadMockRenderer::with_free_frames(256);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 0);
    assert!(seq.is_empty());
    assert_eq!(seq.len(), 0);
}

#[test]
fn sequence_consumes_audio_until_buffer_empty() {
    // Buffer with 100 frames, renderer with 50 free frames per step.
    // Each step consumes 50 frames, so 2 steps to drain.
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let renderer = OutputThreadMockRenderer::with_free_frames(50);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 3);
    // First step: 50 audio frames
    // Second step: 50 audio frames
    // Third step: empty open buffer → silence (no sleep break, but max_steps=3)
    assert_eq!(seq.len(), 3);
    let first = &seq.steps[0];
    assert_eq!(first.plan.action, OutputThreadRenderAction::RenderAudio);
    assert_eq!(first.plan.frames_to_read, 50);
    let second = &seq.steps[1];
    assert_eq!(second.plan.action, OutputThreadRenderAction::RenderAudio);
    assert_eq!(second.plan.frames_to_read, 50);
    let third = &seq.steps[2];
    assert_eq!(third.plan.action, OutputThreadRenderAction::RenderSilence);
}

#[test]
fn sequence_stops_on_sleep() {
    let buffer = OutputThreadMockBuffer::empty_open();
    let renderer = OutputThreadMockRenderer::full();
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 100);
    assert!(!seq.is_empty());
    let last = seq.last().unwrap();
    assert!(last.should_sleep());
}

#[test]
fn sequence_stops_on_exit() {
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let renderer = OutputThreadMockRenderer::with_free_frames(256);
    let control = OutputThreadControlSnapshot {
        shutdown_requested: true,
        ..running_control()
    };
    let seq = run_mock_sequence(buffer, renderer, control, 100);
    assert_eq!(seq.len(), 1);
    let last = seq.last().unwrap();
    assert!(last.should_exit());
}

#[test]
fn sequence_accumulates_rendered_frames() {
    let buffer = OutputThreadMockBuffer::with_frames(300);
    let renderer = OutputThreadMockRenderer::with_free_frames(100);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 10);
    // 3 steps of 100 audio frames, then silence + sleep
    assert!(seq.total_rendered_frames() >= 300);
}

#[test]
fn sequence_accumulates_silence_frames() {
    let buffer = OutputThreadMockBuffer::empty_open();
    let renderer = OutputThreadMockRenderer::with_free_frames(64);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 5);
    // Empty open buffer → silence plan, then sleep
    assert!(seq.total_silence_frames() > 0);
}

#[test]
fn sequence_reports_validation_errors() {
    // All plans from plan_consumer_step should be valid
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let renderer = OutputThreadMockRenderer::with_free_frames(256);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 5);
    assert!(!seq.has_validation_errors());
}

#[test]
fn sequence_last_returns_final_step() {
    let buffer = OutputThreadMockBuffer::with_frames(50);
    let renderer = OutputThreadMockRenderer::with_free_frames(256);
    let control = running_control();
    let seq = run_mock_sequence(buffer, renderer, control, 10);
    let last = seq.last().unwrap();
    // After consuming 50 frames, buffer is empty open → silence
    assert_eq!(last.plan.action, OutputThreadRenderAction::RenderSilence);
}
