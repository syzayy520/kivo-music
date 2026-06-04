use super::output_thread_mock_scenarios;
use super::output_thread_state::OutputThreadState;

#[test]
fn normal_audio_factory_has_expected_name() {
    let s = output_thread_mock_scenarios::normal_audio();
    assert_eq!(s.name, "normal_audio");
    assert!(s.has_capacity());
}

#[test]
fn empty_running_factory_has_expected_name() {
    let s = output_thread_mock_scenarios::empty_running();
    assert_eq!(s.name, "empty_running");
}

#[test]
fn no_capacity_factory_has_no_capacity() {
    let s = output_thread_mock_scenarios::no_capacity();
    assert_eq!(s.name, "no_capacity");
    assert!(!s.has_capacity());
}

#[test]
fn shutdown_requested_factory_requests_shutdown() {
    let s = output_thread_mock_scenarios::shutdown_requested();
    assert_eq!(s.name, "shutdown_requested");
    assert!(s.control.shutdown_requested);
}

#[test]
fn paused_factories_are_paused() {
    let empty = output_thread_mock_scenarios::paused_empty();
    assert!(empty.control.paused);
    let with_frames = output_thread_mock_scenarios::paused_with_frames();
    assert!(with_frames.control.paused);
}

#[test]
fn flush_empty_factory_requests_flush() {
    let s = output_thread_mock_scenarios::flush_empty();
    assert_eq!(s.name, "flush_empty");
    assert!(s.control.flush_requested);
}

#[test]
fn closed_factories_are_closed() {
    let empty = output_thread_mock_scenarios::closed_empty();
    assert!(empty.buffer.snapshot().is_closed);
    assert_eq!(empty.buffer.snapshot().available_frames, 0);
    let remaining = output_thread_mock_scenarios::closed_with_remaining();
    assert!(remaining.buffer.snapshot().is_closed);
    assert!(remaining.buffer.snapshot().available_frames > 0);
}

#[test]
fn non_running_factory_is_not_running() {
    let s = output_thread_mock_scenarios::non_running();
    assert_eq!(s.name, "non_running");
    assert_ne!(s.control.state, OutputThreadState::Running);
}

#[test]
fn all_factories_are_bounded() {
    let scenarios = [
        output_thread_mock_scenarios::normal_audio(),
        output_thread_mock_scenarios::empty_running(),
        output_thread_mock_scenarios::no_capacity(),
        output_thread_mock_scenarios::shutdown_requested(),
        output_thread_mock_scenarios::paused_empty(),
        output_thread_mock_scenarios::paused_with_frames(),
        output_thread_mock_scenarios::flush_empty(),
        output_thread_mock_scenarios::closed_empty(),
        output_thread_mock_scenarios::closed_with_remaining(),
        output_thread_mock_scenarios::non_running(),
    ];
    for s in scenarios {
        assert!(s.is_bounded(), "{} should be bounded", s.name);
    }
}
