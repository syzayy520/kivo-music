use super::native_pipeline_clock::NativePipelineClock;

#[test]
fn clock_starts_with_default_state() {
    let clock = NativePipelineClock::new();
    assert_eq!(clock.position_ms(), 0);
    assert!(!clock.is_started());
    assert!(!clock.is_paused());
}

#[test]
fn clock_start_at_sets_position_and_started() {
    let mut clock = NativePipelineClock::new();
    clock.start_at(1000);
    assert_eq!(clock.position_ms(), 1000);
    assert!(clock.is_started());
    assert!(!clock.is_paused());
}

#[test]
fn clock_pause_sets_paused_state() {
    let mut clock = NativePipelineClock::new();
    clock.start_at(0);
    clock.pause();
    assert!(clock.is_paused());
}

#[test]
fn clock_resume_clears_paused_state() {
    let mut clock = NativePipelineClock::new();
    clock.start_at(0);
    clock.pause();
    clock.resume();
    assert!(!clock.is_paused());
}

#[test]
fn clock_reset_returns_to_default_state() {
    let mut clock = NativePipelineClock::new();
    clock.start_at(5000);
    clock.pause();
    clock.reset();
    assert_eq!(clock.position_ms(), 0);
    assert!(!clock.is_started());
    assert!(!clock.is_paused());
}

#[test]
fn clock_set_position_updates_position() {
    let mut clock = NativePipelineClock::new();
    clock.start_at(0);
    clock.set_position(3000);
    assert_eq!(clock.position_ms(), 3000);
}
