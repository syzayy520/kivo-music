//! Render source cursor and snapshot tests.
//!
//! Tests for SourceCursor and SourceSnapshot types.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    SourceCursor, SourceSnapshot,
};

// ===== SourceCursor tests =====

#[test]
fn source_cursor_default() {
    let cursor = SourceCursor::default();
    assert_eq!(cursor.position_frames, 0);
    assert_eq!(cursor.total_frames, 0);
    assert_eq!(cursor.sample_rate, 0);
    assert_eq!(cursor.channel_count, 0);
}

#[test]
fn source_cursor_equality() {
    let a = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(a, b);
}

#[test]
fn source_cursor_inequality() {
    let a = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = SourceCursor {
        position_frames: 200,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_ne!(a, b);
}

#[test]
fn source_cursor_is_at_end_unknown_total() {
    let cursor = SourceCursor {
        position_frames: 100,
        total_frames: 0,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(!cursor.is_at_end());
}

#[test]
fn source_cursor_is_at_end_not_reached() {
    let cursor = SourceCursor {
        position_frames: 500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(!cursor.is_at_end());
}

#[test]
fn source_cursor_is_at_end_reached() {
    let cursor = SourceCursor {
        position_frames: 1000,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(cursor.is_at_end());
}

#[test]
fn source_cursor_is_at_end_past_end() {
    let cursor = SourceCursor {
        position_frames: 1500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(cursor.is_at_end());
}

#[test]
fn source_cursor_frames_remaining_unknown() {
    let cursor = SourceCursor {
        position_frames: 100,
        total_frames: 0,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(cursor.frames_remaining(), None);
}

#[test]
fn source_cursor_frames_remaining_known() {
    let cursor = SourceCursor {
        position_frames: 400,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(cursor.frames_remaining(), Some(600));
}

#[test]
fn source_cursor_frames_remaining_at_end() {
    let cursor = SourceCursor {
        position_frames: 1000,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(cursor.frames_remaining(), Some(0));
}

#[test]
fn source_cursor_frames_remaining_saturating() {
    let cursor = SourceCursor {
        position_frames: 1500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(cursor.frames_remaining(), Some(0));
}

#[test]
fn source_cursor_clone() {
    let original = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn source_cursor_debug() {
    let cursor = SourceCursor::default();
    let debug = format!("{:?}", cursor);
    assert!(debug.contains("SourceCursor"));
}

// ===== SourceSnapshot tests =====

#[test]
fn source_snapshot_default() {
    let snap = SourceSnapshot::default();
    assert_eq!(snap.requests_accepted, 0);
    assert_eq!(snap.packets_provided, 0);
    assert_eq!(snap.frames_read, 0);
    assert_eq!(snap.bytes_read, 0);
    assert_eq!(snap.errors, 0);
    assert!(!snap.is_exhausted);
    assert!(!snap.is_ready);
}

#[test]
fn source_snapshot_equality() {
    let a = SourceSnapshot {
        requests_accepted: 10,
        packets_provided: 8,
        frames_read: 8192,
        bytes_read: 32768,
        errors: 1,
        is_exhausted: false,
        is_ready: true,
    };
    let b = SourceSnapshot {
        requests_accepted: 10,
        packets_provided: 8,
        frames_read: 8192,
        bytes_read: 32768,
        errors: 1,
        is_exhausted: false,
        is_ready: true,
    };
    assert_eq!(a, b);
}

#[test]
fn source_snapshot_inequality() {
    let a = SourceSnapshot {
        requests_accepted: 10,
        ..Default::default()
    };
    let b = SourceSnapshot {
        requests_accepted: 20,
        ..Default::default()
    };
    assert_ne!(a, b);
}

#[test]
fn source_snapshot_clone() {
    let original = SourceSnapshot {
        requests_accepted: 10,
        packets_provided: 8,
        frames_read: 8192,
        bytes_read: 32768,
        errors: 1,
        is_exhausted: true,
        is_ready: false,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn source_snapshot_debug() {
    let snap = SourceSnapshot::default();
    let debug = format!("{:?}", snap);
    assert!(debug.contains("SourceSnapshot"));
}

#[test]
fn source_snapshot_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let a = SourceSnapshot {
        requests_accepted: 10,
        ..Default::default()
    };
    let b = SourceSnapshot {
        requests_accepted: 10,
        ..Default::default()
    };
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}
