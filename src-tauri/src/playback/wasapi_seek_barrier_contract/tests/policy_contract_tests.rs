//! Policy contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::reset_policy::WasapiResetPolicy;

#[test]
fn playing_seek_requires_device_reset_or_stop_reset_start_policy() {
    assert!(!WasapiResetPolicy::FlushOnly.can_satisfy_playing_seek_device_barrier());
    assert!(WasapiResetPolicy::ResetDevice.can_satisfy_playing_seek_device_barrier());
    assert!(WasapiResetPolicy::StopResetStart.can_satisfy_playing_seek_device_barrier());
}

#[test]
fn flush_only_is_not_playing_seek_device_buffer_safe() {
    let policy = WasapiResetPolicy::FlushOnly;
    assert!(policy.is_flush_only());
    assert!(!policy.requires_device_reset());
    assert!(!policy.requires_stop_start_cycle());
    assert!(!policy.can_satisfy_playing_seek_device_barrier());
    let reset = WasapiResetPolicy::ResetDevice;
    assert!(!reset.is_flush_only());
    assert!(reset.requires_device_reset());
    assert!(!reset.requires_stop_start_cycle());
    assert!(reset.can_satisfy_playing_seek_device_barrier());
}
