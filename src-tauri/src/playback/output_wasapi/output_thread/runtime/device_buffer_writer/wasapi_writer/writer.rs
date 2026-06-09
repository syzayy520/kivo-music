//! WASAPI device buffer writer type scaffold.
//!
//! Placeholder implementation of DeviceBufferWriter for WASAPI.
//! No real WASAPI calls, no COM objects, no audio data processing.

use super::super::{
    DeviceBufferWriter, WriteError, WriteRequest, WriteResult, WriterCursor, WriterState,
};
use super::{WasapiDeviceBufferWriterConfig, WasapiDeviceBufferWriterState};

/// WASAPI device buffer writer type scaffold.
///
/// Placeholder implementation that satisfies the `DeviceBufferWriter` trait.
/// No real WASAPI calls are made; all methods return safe placeholder values.
#[derive(Debug)]
pub struct WasapiDeviceBufferWriter {
    config: WasapiDeviceBufferWriterConfig,
    state: WasapiDeviceBufferWriterState,
}

impl WasapiDeviceBufferWriter {
    /// Creates a new WASAPI device buffer writer with the given configuration.
    pub fn new(config: WasapiDeviceBufferWriterConfig) -> Self {
        Self {
            config,
            state: WasapiDeviceBufferWriterState::new(),
        }
    }

    /// Creates a new writer with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(WasapiDeviceBufferWriterConfig::default())
    }

    /// Returns a reference to the writer configuration.
    pub fn config(&self) -> &WasapiDeviceBufferWriterConfig {
        &self.config
    }

    /// Returns a reference to the internal state.
    pub fn internal_state(&self) -> &WasapiDeviceBufferWriterState {
        &self.state
    }
}

impl DeviceBufferWriter for WasapiDeviceBufferWriter {
    fn process_request(&mut self, request: &WriteRequest) -> Result<WriteResult, WriteError> {
        if self.state.is_closed {
            return Err(WriteError::DeviceClosed);
        }

        match request {
            WriteRequest::WritePacket { .. } => {
                // Scaffold: no real write, return Noop.
                let result = WriteResult::Noop;
                self.state.last_result = result.clone();
                Ok(result)
            }
            WriteRequest::Flush => {
                let result = WriteResult::Noop;
                self.state.last_result = result.clone();
                Ok(result)
            }
            WriteRequest::Close => {
                self.state.is_closed = true;
                let result = WriteResult::Noop;
                self.state.last_result = result.clone();
                Ok(result)
            }
            WriteRequest::Noop => {
                let result = WriteResult::Noop;
                self.state.last_result = result.clone();
                Ok(result)
            }
        }
    }

    fn snapshot(&self) -> WriterState {
        WriterState {
            is_closed: self.state.is_closed,
            is_ready: !self.state.is_closed,
            buffer_capacity_frames: self.config.capacity_frames,
            ..WriterState::default()
        }
    }

    fn cursor(&self) -> WriterCursor {
        WriterCursor {
            buffer_capacity: self.config.capacity_frames,
            sample_rate: self.config.sample_rate,
            channel_count: self.config.channels,
            ..WriterCursor::default()
        }
    }

    fn is_ready(&self) -> bool {
        !self.state.is_closed
    }

    fn is_closed(&self) -> bool {
        self.state.is_closed
    }

    fn reset(&mut self) {
        self.state = WasapiDeviceBufferWriterState::new();
    }
}
