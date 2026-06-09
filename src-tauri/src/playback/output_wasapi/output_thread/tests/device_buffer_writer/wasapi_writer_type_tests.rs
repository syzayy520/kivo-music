//! Type-level tests for WASAPI device buffer writer.
//!
//! Verifies:
//! 1. WasapiDeviceBufferWriter can be constructed
//! 2. Initial state is well-defined
//! 3. Implements DeviceBufferWriter trait
//! 4. Close behavior is safe
//! 5. No real WASAPI calls are made
//! 6. Config/state types do not hold real WASAPI resources

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig,
    WasapiDeviceBufferWriterState, WriteError, WriteRequest, WriteResult,
};

#[test]
fn wasapi_writer_can_be_constructed_with_config() {
    let config = WasapiDeviceBufferWriterConfig::new(2048, 2, 48000);
    let writer = WasapiDeviceBufferWriter::new(config);
    assert!(!writer.is_closed());
    assert!(writer.is_ready());
}

#[test]
fn wasapi_writer_can_be_constructed_with_defaults() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert!(!writer.is_closed());
    assert!(writer.is_ready());
    assert_eq!(writer.config().capacity_frames(), 1024);
    assert_eq!(writer.config().channels(), 2);
    assert_eq!(writer.config().sample_rate(), 44100);
}

#[test]
fn wasapi_writer_initial_state_is_well_defined() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let state = writer.internal_state();
    assert!(!state.is_closed());
    assert_eq!(state.buffer_fill_frames(), 0);
    assert_eq!(state.frames_written(), 0);
    assert_eq!(state.bytes_written(), 0);
    assert_eq!(state.write_attempts(), 0);
    assert_eq!(state.would_block_count(), 0);
    assert_eq!(state.flush_count(), 0);
    assert_eq!(state.last_result(), &WriteResult::Noop);
}

#[test]
fn wasapi_writer_implements_device_buffer_writer_trait() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // snapshot returns valid state
    let snapshot = writer.snapshot();
    assert!(!snapshot.is_closed);
    assert!(snapshot.is_ready);
    assert_eq!(snapshot.buffer_capacity_frames, 1024);

    // cursor returns valid cursor
    let cursor = writer.cursor();
    assert_eq!(cursor.buffer_capacity, 1024);
    assert_eq!(cursor.sample_rate, 44100);
    assert_eq!(cursor.channel_count, 2);

    // is_ready / is_closed
    assert!(writer.is_ready());
    assert!(!writer.is_closed());

    // process_request returns Noop
    let result = writer.process_request(&WriteRequest::Noop).unwrap();
    assert_eq!(result, WriteResult::Noop);

    // reset works
    writer.reset();
    assert!(!writer.is_closed());
}

#[test]
fn wasapi_writer_close_behavior_is_safe() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Close returns Ok(Noop)
    let result = writer.process_request(&WriteRequest::Close).unwrap();
    assert_eq!(result, WriteResult::Noop);

    // After close: is_closed = true, is_ready = false
    assert!(writer.is_closed());
    assert!(!writer.is_ready());

    // After close: snapshot reflects closed state
    let snapshot = writer.snapshot();
    assert!(snapshot.is_closed);
    assert!(!snapshot.is_ready);

    // After close: process_request returns DeviceClosed
    let err = writer.process_request(&WriteRequest::Noop).unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

#[test]
fn wasapi_writer_close_then_write_returns_error() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Close).unwrap();

    let err = writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

#[test]
fn wasapi_writer_reset_restores_state() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Close).unwrap();
    assert!(writer.is_closed());

    writer.reset();
    assert!(!writer.is_closed());
    assert!(writer.is_ready());

    // After reset, can process requests again
    let result = writer.process_request(&WriteRequest::Noop).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

#[test]
fn wasapi_writer_flush_returns_noop() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let result = writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

#[test]
fn wasapi_config_does_not_hold_wasm_resources() {
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);

    // Config is pure data: can be cloned, compared, debug-printed
    let config2 = config.clone();
    assert_eq!(config, config2);
    let _debug = format!("{:?}", config);
}

#[test]
fn wasapi_state_does_not_hold_wasm_resources() {
    let state = WasapiDeviceBufferWriterState::new();

    // State is pure data: can be cloned, debug-printed
    let state2 = state.clone();
    assert!(!state2.is_closed());
    let _debug = format!("{:?}", state);
}
