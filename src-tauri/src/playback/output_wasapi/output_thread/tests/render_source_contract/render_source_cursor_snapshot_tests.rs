//! Render source cursor and snapshot tests.
//!
//! Tests for SourceCursor and SourceSnapshot types.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    SourceCursor, SourceSnapshot,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn source_cursor_default() {
    let cursor = SourceCursor::default();
    assert_eq!(cursor.position_frames, 0);
    assert_eq!(cursor.total_frames, 0);
    assert_eq!(cursor.sample_rate, 0);
    assert_eq!(cursor.channel_count, 0);
}

#[test]
fn source_cursor_equality_and_inequality() {
    let base = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let same = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let diff = SourceCursor {
        position_frames: 200,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(base, same);
    assert_ne!(base, diff);
}

#[test]
fn source_cursor_is_at_end_all_cases() {
    let unknown = SourceCursor {
        position_frames: 100,
        total_frames: 0,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(!unknown.is_at_end());
    let not_reached = SourceCursor {
        position_frames: 500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(!not_reached.is_at_end());
    let reached = SourceCursor {
        position_frames: 1000,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(reached.is_at_end());
    let past_end = SourceCursor {
        position_frames: 1500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert!(past_end.is_at_end());
}

#[test]
fn source_cursor_frames_remaining_all_cases() {
    let unknown = SourceCursor {
        position_frames: 100,
        total_frames: 0,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(unknown.frames_remaining(), None);
    let known = SourceCursor {
        position_frames: 400,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(known.frames_remaining(), Some(600));
    let at_end = SourceCursor {
        position_frames: 1000,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(at_end.frames_remaining(), Some(0));
    let saturating = SourceCursor {
        position_frames: 1500,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(saturating.frames_remaining(), Some(0));
}

#[test]
fn source_cursor_clone_and_debug() {
    let original = SourceCursor {
        position_frames: 100,
        total_frames: 1000,
        sample_rate: 44100,
        channel_count: 2,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
    let debug = format!("{:?}", SourceCursor::default());
    assert!(debug.contains("SourceCursor"));
}

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
fn source_snapshot_equality_and_inequality() {
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
    let c = SourceSnapshot {
        requests_accepted: 20,
        ..Default::default()
    };
    assert_ne!(a, c);
}

#[test]
fn source_snapshot_clone_and_debug() {
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
    let debug = format!("{:?}", SourceSnapshot::default());
    assert!(debug.contains("SourceSnapshot"));
}

#[test]
fn source_snapshot_hash_consistency() {
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
