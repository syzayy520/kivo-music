use super::output_thread_buffer_snapshot::OutputThreadBufferSnapshot;

#[test]
fn default_snapshot_is_empty_and_open() {
    let snap = OutputThreadBufferSnapshot::default();
    assert_eq!(snap.available_frames, 0);
    assert!(!snap.is_closed);
    assert!(snap.is_empty());
    assert!(!snap.has_frames());
    assert!(!snap.can_drain());
}

#[test]
fn snapshot_with_available_frames_has_frames() {
    let snap = OutputThreadBufferSnapshot::new(512, false);
    assert!(snap.has_frames());
    assert!(!snap.is_empty());
    assert!(snap.can_drain());
}

#[test]
fn closed_empty_snapshot_is_exhausted() {
    let snap = OutputThreadBufferSnapshot::new(0, true);
    assert!(snap.is_closed());
    assert!(snap.is_empty());
    assert!(!snap.can_drain());
}

#[test]
fn closed_snapshot_with_frames_can_still_drain() {
    let snap = OutputThreadBufferSnapshot::new(256, true);
    assert!(snap.is_closed());
    assert!(snap.has_frames());
    assert!(snap.can_drain());
}

#[test]
fn constructor_preserves_values() {
    let snap = OutputThreadBufferSnapshot::new(1024, false);
    assert_eq!(snap.available_frames, 1024);
    assert!(!snap.is_closed);
}
