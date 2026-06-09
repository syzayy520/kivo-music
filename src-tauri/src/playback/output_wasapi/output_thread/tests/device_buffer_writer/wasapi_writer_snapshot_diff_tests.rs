//! Snapshot comparison and summary tests.
//!
//! Tests for changed_field_names(), is_same_state(), and summary_line() on WriterState.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
    WriterState,
};

#[test]
fn identical_snapshots_have_no_changes() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snap1 = writer.snapshot();
    let snap2 = writer.snapshot();
    assert!(snap1.changed_field_names(&snap2).is_empty());
    assert!(snap1.is_same_state(&snap2));
}

#[test]
fn default_snapshots_are_same() {
    let snap1: WriterState = Default::default();
    let snap2: WriterState = Default::default();
    assert!(snap1.is_same_state(&snap2));
}

#[test]
fn changed_fields_after_write() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);
    let before = writer.snapshot();

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let after = writer.snapshot();
    let changes = after.changed_field_names(&before);

    assert!(changes.contains(&"requests_accepted"));
    assert!(changes.contains(&"writes_completed"));
    assert!(changes.contains(&"frames_written"));
    assert!(changes.contains(&"bytes_written"));
    assert!(changes.contains(&"buffer_fill_frames"));
    assert!(changes.contains(&"write_streak"));
    assert!(changes.contains(&"max_write_streak"));
    // lifecycle changes from Empty to Partial
    assert!(changes.contains(&"lifecycle"));
}

#[test]
fn changed_fields_after_would_block() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    let before = writer.snapshot();

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let after = writer.snapshot();
    let changes = after.changed_field_names(&before);

    assert!(changes.contains(&"would_block_count"));
    assert!(changes.contains(&"consecutive_would_blocks"));
    assert!(changes.contains(&"max_consecutive_would_blocks"));
    assert!(changes.contains(&"write_streak"));
}

#[test]
fn changed_fields_after_flush() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    let before = writer.snapshot();

    writer.process_request(&WriteRequest::Flush).unwrap();

    let after = writer.snapshot();
    let changes = after.changed_field_names(&before);

    assert!(changes.contains(&"buffer_fill_frames"));
    assert!(changes.contains(&"flush_count"));
    // lifecycle changes from Partial to Empty
    assert!(changes.contains(&"lifecycle"));
}

#[test]
fn changed_fields_after_close() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);
    let before = writer.snapshot();

    writer.process_request(&WriteRequest::Close).unwrap();

    let after = writer.snapshot();
    let changes = after.changed_field_names(&before);

    assert!(changes.contains(&"is_closed"));
    assert!(changes.contains(&"is_ready"));
    assert!(changes.contains(&"lifecycle"));
}

#[test]
fn summary_line_contains_key_info() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(500, 44100, 2))
        .unwrap();

    let summary = writer.snapshot().summary_line();
    // Should contain lifecycle, fill percentage, streak, and health status
    assert!(summary.contains("Partial"));
    assert!(summary.contains("fill="));
    assert!(summary.contains("streak=1/1"));
    assert!(summary.contains("healthy=true"));
    assert!(summary.contains("pressure="));
}

#[test]
fn summary_line_for_healthy_initial_state() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let summary = writer.snapshot().summary_line();
    assert!(summary.contains("Empty"));
    assert!(summary.contains("fill=0%"));
    assert!(summary.contains("streak=0/0"));
    assert!(summary.contains("healthy=true"));
}

#[test]
fn is_same_state_returns_false_when_different() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);
    let snap1 = writer.snapshot();

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap2 = writer.snapshot();
    assert!(!snap1.is_same_state(&snap2));
}

#[test]
fn changed_fields_symmetric() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);
    let before = writer.snapshot();

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let after = writer.snapshot();
    // Both directions should report the same changed fields
    let changes_forward = after.changed_field_names(&before);
    let changes_backward = before.changed_field_names(&after);
    assert_eq!(changes_forward.len(), changes_backward.len());
    for field in &changes_forward {
        assert!(changes_backward.contains(field));
    }
}
