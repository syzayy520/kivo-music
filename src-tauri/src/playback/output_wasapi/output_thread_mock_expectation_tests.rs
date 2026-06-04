use super::output_thread_mock_expectation::OutputThreadMockExpectation;
use super::output_thread_render_plan::OutputThreadRenderAction;

fn exit_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Exit,
        1,
        5,
        0,
        0,
        true,
        false,
        true,
    )
}

fn sleep_expectation() -> OutputThreadMockExpectation {
    OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Sleep,
        1,
        10,
        0,
        0,
        false,
        true,
        true,
    )
}

#[test]
fn expectation_new_preserves_fields() {
    let e = OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        2,
        8,
        100,
        50,
        false,
        false,
        true,
    );
    assert_eq!(
        e.expected_final_action,
        OutputThreadRenderAction::RenderSilence
    );
    assert_eq!(e.min_steps, 2);
    assert_eq!(e.max_steps, 8);
    assert_eq!(e.min_rendered_frames, 100);
    assert_eq!(e.min_silence_frames, 50);
    assert!(!e.should_end);
    assert!(!e.should_sleep);
    assert!(e.should_be_valid);
}

#[test]
fn expectation_detects_exit() {
    let e = exit_expectation();
    assert!(e.expects_exit());
    assert!(!e.expects_sleep());
}

#[test]
fn expectation_detects_sleep() {
    let e = sleep_expectation();
    assert!(e.expects_sleep());
    assert!(!e.expects_exit());
}

#[test]
fn expectation_allows_step_count_inside_range() {
    let e = OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Sleep,
        2,
        5,
        0,
        0,
        false,
        true,
        true,
    );
    assert!(e.allows_step_count(2));
    assert!(e.allows_step_count(3));
    assert!(e.allows_step_count(5));
}

#[test]
fn expectation_rejects_step_count_outside_range() {
    let e = OutputThreadMockExpectation::new(
        OutputThreadRenderAction::Sleep,
        2,
        5,
        0,
        0,
        false,
        true,
        true,
    );
    assert!(!e.allows_step_count(1));
    assert!(!e.allows_step_count(6));
}

#[test]
fn expectation_allows_rendered_frames_above_minimum() {
    let e = OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderAudio,
        1,
        10,
        100,
        0,
        false,
        false,
        true,
    );
    assert!(e.allows_rendered_frames(100));
    assert!(e.allows_rendered_frames(200));
    assert!(!e.allows_rendered_frames(50));
}

#[test]
fn expectation_allows_silence_frames_above_minimum() {
    let e = OutputThreadMockExpectation::new(
        OutputThreadRenderAction::RenderSilence,
        1,
        10,
        0,
        50,
        false,
        false,
        true,
    );
    assert!(e.allows_silence_frames(50));
    assert!(e.allows_silence_frames(100));
    assert!(!e.allows_silence_frames(25));
}
