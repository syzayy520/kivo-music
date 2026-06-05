use super::scenarios;
use super::super::output_thread_state::OutputThreadState;

#[test]
fn normal_audio_factory_has_expected_name() {
    let s = scenarios::normal_audio();
    assert_eq!(s.name, "normal_audio");
    assert!(s.has_capacity());
}

#[test]
fn empty_running_factory_has_expected_name() {
    let s = scenarios::empty_running();
    assert_eq!(s.name, "empty_running");
}

#[test]
fn no_capacity_factory_has_no_capacity() {
    let s = scenarios::no_capacity();
    assert_eq!(s.name, "no_capacity");
    assert!(!s.has_capacity());
}

#[test]
fn shutdown_requested_factory_requests_shutdown() {
    let s = scenarios::shutdown_requested();
    assert_eq!(s.name, "shutdown_requested");
    assert!(s.control.shutdown_requested);
}

#[test]
fn paused_factories_are_paused() {
    let empty = scenarios::paused_empty();
    assert!(empty.control.paused);
    let with_frames = scenarios::paused_with_frames();
    assert!(with_frames.control.paused);
}

#[test]
fn flush_empty_factory_requests_flush() {
    let s = scenarios::flush_empty();
    assert_eq!(s.name, "flush_empty");
    assert!(s.control.flush_requested);
}

#[test]
fn closed_factories_are_closed() {
    let empty = scenarios::closed_empty();
    assert!(empty.buffer.snapshot().is_closed);
    assert_eq!(empty.buffer.snapshot().available_frames, 0);
    let remaining = scenarios::closed_with_remaining();
    assert!(remaining.buffer.snapshot().is_closed);
    assert!(remaining.buffer.snapshot().available_frames > 0);
}

#[test]
fn non_running_factory_is_not_running() {
    let s = scenarios::non_running();
    assert_eq!(s.name, "non_running");
    assert_ne!(s.control.state, OutputThreadState::Running);
}

#[test]
fn all_factories_are_bounded() {
    let scenarios = [
        scenarios::normal_audio(),
        scenarios::empty_running(),
        scenarios::no_capacity(),
        scenarios::shutdown_requested(),
        scenarios::paused_empty(),
        scenarios::paused_with_frames(),
        scenarios::flush_empty(),
        scenarios::closed_empty(),
        scenarios::closed_with_remaining(),
        scenarios::non_running(),
    ];
    for s in scenarios {
        assert!(s.is_bounded(), "{} should be bounded", s.name);
    }
}
