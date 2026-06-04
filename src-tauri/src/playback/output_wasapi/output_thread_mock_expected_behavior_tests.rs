use super::output_thread_mock_expected_behavior::*;
use super::output_thread_render_plan::OutputThreadRenderAction;

const KNOWN_NAMES: &[&str] = &[
    "normal_audio",
    "empty_running",
    "no_capacity",
    "shutdown_requested",
    "paused_empty",
    "paused_with_frames",
    "flush_empty",
    "closed_empty",
    "closed_with_remaining",
    "non_running",
];

#[test]
fn expectation_for_known_names_returns_some() {
    for name in KNOWN_NAMES {
        assert!(expectation_for_name(name).is_some(), "missing: {name}");
    }
}

#[test]
fn expectation_for_unknown_name_returns_none() {
    assert!(expectation_for_name("unknown").is_none());
    assert!(expectation_for_name("").is_none());
}

#[test]
fn shutdown_expectation_expects_exit() {
    let e = expectation_for_name("shutdown_requested").unwrap();
    assert!(e.expects_exit());
    assert_eq!(e.expected_final_action, OutputThreadRenderAction::Exit);
}

#[test]
fn no_capacity_expectation_expects_sleep() {
    let e = expectation_for_name("no_capacity").unwrap();
    assert!(e.expects_sleep());
    assert_eq!(e.expected_final_action, OutputThreadRenderAction::Sleep);
}

#[test]
fn normal_audio_expectation_requires_rendered_frames() {
    let e = expectation_for_name("normal_audio").unwrap();
    assert!(e.min_rendered_frames > 0);
}

#[test]
fn silence_scenarios_require_silence_frames() {
    for name in &[
        "empty_running",
        "paused_empty",
        "paused_with_frames",
        "flush_empty",
    ] {
        let e = expectation_for_name(name).unwrap();
        assert!(
            e.min_silence_frames > 0,
            "{name} should require silence frames"
        );
    }
}

#[test]
fn closed_empty_expectation_expects_exit() {
    let e = expectation_for_name("closed_empty").unwrap();
    assert!(e.expects_exit());
    assert_eq!(e.expected_final_action, OutputThreadRenderAction::Exit);
}

#[test]
fn non_running_expectation_expects_sleep() {
    let e = expectation_for_name("non_running").unwrap();
    assert!(e.expects_sleep());
    assert_eq!(e.expected_final_action, OutputThreadRenderAction::Sleep);
}
