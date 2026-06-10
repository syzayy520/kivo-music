//! Runtime mode type and integration tests for device buffer writer.
//!
//! Tests that RuntimeMode, RuntimeKind, and Readiness enums work correctly,
//! and that the WasapiDeviceBufferWriter properly exposes runtime mode in snapshots.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::wasapi_writer::{
    runtime_mode::{Readiness, RuntimeKind, RuntimeMode},
    WasapiDeviceBufferWriter,
};
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WriterState,
};

// ── RuntimeMode enum tests ──────────────────────────────────────────────────

#[test]
fn runtime_mode_default_is_placeholder() {
    let mode = RuntimeMode::default();
    assert_eq!(mode, RuntimeMode::Placeholder);
}

#[test]
fn runtime_mode_is_placeholder() {
    assert!(RuntimeMode::Placeholder.is_placeholder());
    assert!(!RuntimeMode::Real.is_placeholder());
    assert!(!RuntimeMode::NotConnected.is_placeholder());
    assert!(!RuntimeMode::BoundaryUnavailable.is_placeholder());
}

#[test]
fn runtime_mode_is_real() {
    assert!(!RuntimeMode::Placeholder.is_real());
    assert!(RuntimeMode::Real.is_real());
    assert!(!RuntimeMode::NotConnected.is_real());
    assert!(!RuntimeMode::BoundaryUnavailable.is_real());
}

#[test]
fn runtime_mode_can_accept_requests() {
    assert!(RuntimeMode::Placeholder.can_accept_requests());
    assert!(RuntimeMode::Real.can_accept_requests());
    assert!(!RuntimeMode::NotConnected.can_accept_requests());
    assert!(!RuntimeMode::BoundaryUnavailable.can_accept_requests());
}

#[test]
fn runtime_mode_display() {
    assert_eq!(format!("{:?}", RuntimeMode::Placeholder), "Placeholder");
    assert_eq!(format!("{:?}", RuntimeMode::Real), "Real");
    assert_eq!(format!("{:?}", RuntimeMode::NotConnected), "NotConnected");
    assert_eq!(
        format!("{:?}", RuntimeMode::BoundaryUnavailable),
        "BoundaryUnavailable"
    );
}

#[test]
fn runtime_mode_clone_copy() {
    let mode = RuntimeMode::Real;
    let cloned = mode;
    assert_eq!(mode, cloned);
}

#[test]
fn runtime_mode_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    RuntimeMode::Placeholder.hash(&mut h1);
    RuntimeMode::Placeholder.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ── RuntimeKind enum tests ──────────────────────────────────────────────────

#[test]
fn runtime_kind_default_is_simulated() {
    let kind = RuntimeKind::default();
    assert_eq!(kind, RuntimeKind::Simulated);
}

#[test]
fn runtime_kind_is_simulated() {
    assert!(RuntimeKind::Simulated.is_simulated());
    assert!(!RuntimeKind::Wasapi.is_simulated());
    assert!(!RuntimeKind::Unknown.is_simulated());
}

#[test]
fn runtime_kind_is_wasapi() {
    assert!(!RuntimeKind::Simulated.is_wasapi());
    assert!(RuntimeKind::Wasapi.is_wasapi());
    assert!(!RuntimeKind::Unknown.is_wasapi());
}

#[test]
fn runtime_kind_display() {
    assert_eq!(format!("{:?}", RuntimeKind::Simulated), "Simulated");
    assert_eq!(format!("{:?}", RuntimeKind::Wasapi), "Wasapi");
    assert_eq!(format!("{:?}", RuntimeKind::Unknown), "Unknown");
}

#[test]
fn runtime_kind_clone_copy() {
    let kind = RuntimeKind::Wasapi;
    let cloned = kind;
    assert_eq!(kind, cloned);
}

// ── Readiness enum tests ────────────────────────────────────────────────────

#[test]
fn readiness_default_is_ready() {
    let readiness = Readiness::default();
    assert_eq!(readiness, Readiness::Ready);
}

#[test]
fn readiness_is_ready() {
    assert!(Readiness::Ready.is_ready());
    assert!(!Readiness::NotReady.is_ready());
    assert!(!Readiness::TemporarilyUnavailable.is_ready());
    assert!(!Readiness::PermanentlyUnavailable.is_ready());
}

#[test]
fn readiness_is_not_ready() {
    assert!(!Readiness::Ready.is_not_ready());
    assert!(Readiness::NotReady.is_not_ready());
    assert!(!Readiness::TemporarilyUnavailable.is_not_ready());
    assert!(!Readiness::PermanentlyUnavailable.is_not_ready());
}

#[test]
fn readiness_is_temporarily_unavailable() {
    assert!(!Readiness::Ready.is_temporarily_unavailable());
    assert!(!Readiness::NotReady.is_temporarily_unavailable());
    assert!(Readiness::TemporarilyUnavailable.is_temporarily_unavailable());
    assert!(!Readiness::PermanentlyUnavailable.is_temporarily_unavailable());
}

#[test]
fn readiness_is_permanently_unavailable() {
    assert!(!Readiness::Ready.is_permanently_unavailable());
    assert!(!Readiness::NotReady.is_permanently_unavailable());
    assert!(!Readiness::TemporarilyUnavailable.is_permanently_unavailable());
    assert!(Readiness::PermanentlyUnavailable.is_permanently_unavailable());
}

#[test]
fn readiness_can_become_ready() {
    assert!(!Readiness::Ready.can_become_ready());
    assert!(Readiness::NotReady.can_become_ready());
    assert!(Readiness::TemporarilyUnavailable.can_become_ready());
    assert!(!Readiness::PermanentlyUnavailable.can_become_ready());
}

#[test]
fn readiness_display() {
    assert_eq!(format!("{:?}", Readiness::Ready), "Ready");
    assert_eq!(format!("{:?}", Readiness::NotReady), "NotReady");
    assert_eq!(
        format!("{:?}", Readiness::TemporarilyUnavailable),
        "TemporarilyUnavailable"
    );
    assert_eq!(
        format!("{:?}", Readiness::PermanentlyUnavailable),
        "PermanentlyUnavailable"
    );
}

#[test]
fn readiness_clone_copy() {
    let readiness = Readiness::Ready;
    let cloned = readiness;
    assert_eq!(readiness, cloned);
}

// ── WasapiDeviceBufferWriter runtime mode integration ───────────────────────

#[test]
fn wasapi_writer_default_runtime_mode_is_placeholder() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let state = writer.internal_state();
    assert_eq!(state.runtime_mode(), RuntimeMode::Placeholder);
    assert!(state.is_placeholder());
    assert!(!state.is_real_runtime());
}

#[test]
fn wasapi_writer_default_readiness_is_ready() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let state = writer.internal_state();
    assert_eq!(state.readiness(), Readiness::Ready);
    assert!(state.is_runtime_ready());
}

#[test]
fn wasapi_writer_set_runtime_mode() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_runtime_mode(RuntimeMode::Real);
    assert_eq!(writer.internal_state().runtime_mode(), RuntimeMode::Real);
    assert!(!writer.internal_state().is_placeholder());
    assert!(writer.internal_state().is_real_runtime());
}

#[test]
fn wasapi_writer_set_readiness() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_readiness(Readiness::NotReady);
    assert_eq!(writer.internal_state().readiness(), Readiness::NotReady);
    assert!(!writer.internal_state().is_runtime_ready());
}

#[test]
fn snapshot_contains_runtime_mode() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.runtime_mode, RuntimeMode::Placeholder);
}

#[test]
fn snapshot_contains_readiness() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.readiness, Readiness::Ready);
}

#[test]
fn snapshot_reflects_runtime_mode_change() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_runtime_mode(RuntimeMode::Real);
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.runtime_mode, RuntimeMode::Real);
}

#[test]
fn snapshot_reflects_readiness_change() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_readiness(Readiness::PermanentlyUnavailable);
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.readiness, Readiness::PermanentlyUnavailable);
}

// ── WriterState runtime mode integration ────────────────────────────────────

#[test]
fn writer_state_default_runtime_mode() {
    let state = WriterState::default();
    assert_eq!(state.runtime_mode, RuntimeMode::Placeholder);
    assert_eq!(state.readiness, Readiness::Ready);
}

#[test]
fn writer_state_changed_field_names_detects_runtime_mode() {
    let state1 = WriterState::default();
    let mut state2 = WriterState::default();
    state2.runtime_mode = RuntimeMode::Real;

    let changes = state1.changed_field_names(&state2);
    assert!(changes.contains(&"runtime_mode"));
}

#[test]
fn writer_state_changed_field_names_detects_readiness() {
    let state1 = WriterState::default();
    let mut state2 = WriterState::default();
    state2.readiness = Readiness::NotReady;

    let changes = state1.changed_field_names(&state2);
    assert!(changes.contains(&"readiness"));
}

#[test]
fn writer_state_is_same_state_detects_runtime_mode_diff() {
    let state1 = WriterState::default();
    let mut state2 = WriterState::default();
    state2.runtime_mode = RuntimeMode::Real;

    assert!(!state1.is_same_state(&state2));
}

#[test]
fn writer_state_summary_line_includes_runtime_mode() {
    let mut state = WriterState::default();
    state.runtime_mode = RuntimeMode::Placeholder;
    state.readiness = Readiness::Ready;

    let summary = state.summary_line();
    assert!(summary.contains("mode=Placeholder"));
    assert!(summary.contains("ready=Ready"));
}

#[test]
fn writer_state_summary_line_reflects_real_mode() {
    let mut state = WriterState::default();
    state.runtime_mode = RuntimeMode::Real;
    state.readiness = Readiness::PermanentlyUnavailable;

    let summary = state.summary_line();
    assert!(summary.contains("mode=Real"));
    assert!(summary.contains("ready=PermanentlyUnavailable"));
}

// ── RuntimeMode × Readiness interaction tests ───────────────────────────────

#[test]
fn placeholder_with_ready_is_valid() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_runtime_mode(RuntimeMode::Placeholder);
    writer.internal_state_mut().set_readiness(Readiness::Ready);
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.runtime_mode, RuntimeMode::Placeholder);
    assert_eq!(snapshot.readiness, Readiness::Ready);
}

#[test]
fn real_with_not_ready_is_valid() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_runtime_mode(RuntimeMode::Real);
    writer
        .internal_state_mut()
        .set_readiness(Readiness::NotReady);
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.runtime_mode, RuntimeMode::Real);
    assert_eq!(snapshot.readiness, Readiness::NotReady);
    assert!(!snapshot.readiness.is_ready());
}

#[test]
fn boundary_unavailable_with_permanently_unavailable_is_valid() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();
    writer
        .internal_state_mut()
        .set_runtime_mode(RuntimeMode::BoundaryUnavailable);
    writer
        .internal_state_mut()
        .set_readiness(Readiness::PermanentlyUnavailable);
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.runtime_mode, RuntimeMode::BoundaryUnavailable);
    assert_eq!(snapshot.readiness, Readiness::PermanentlyUnavailable);
    assert!(!snapshot.runtime_mode.can_accept_requests());
    assert!(!snapshot.readiness.is_ready());
}
