//! WASAPI device buffer writer runtime placeholder.
//!
//! Struct definition, constructors, accessors, and DeviceBufferWriter trait impl.
//! Write packet processing is in packet_write.rs.
//! Request validation is in request_validation.rs.
//! No real WASAPI calls, no COM objects, no audio data processing.

use super::super::{
    DeviceBufferWriter, WriteError, WriteRequest, WriteResult, WriterCursor, WriterState,
};
use super::{WasapiDeviceBufferWriterConfig, WasapiDeviceBufferWriterState};
use crate::playback::output_wasapi::render_client_boundary::RenderClientBoundary;

/// WASAPI device buffer writer runtime placeholder.
///
/// Simulates device buffer write behavior without real WASAPI calls.
/// Supports WritePacket (with simulated fill/WouldBlock), Flush, Close, Noop.
/// No real GetBuffer/ReleaseBuffer/IAudioRenderClient.
///
/// Can optionally hold a RenderClientBoundary instance for delegation.
/// When present, buffer operations may be delegated to the render client.
pub struct WasapiDeviceBufferWriter {
    pub(super) config: WasapiDeviceBufferWriterConfig,
    pub(super) state: WasapiDeviceBufferWriterState,
    pub(super) render_client: Option<Box<dyn RenderClientBoundary>>,
}

impl std::fmt::Debug for WasapiDeviceBufferWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasapiDeviceBufferWriter")
            .field("config", &self.config)
            .field("state", &self.state)
            .field("has_render_client", &self.render_client.is_some())
            .finish()
    }
}

// ── Constructors ──────────────────────────────────────────────────────────

impl WasapiDeviceBufferWriter {
    /// Creates a new WASAPI device buffer writer with the given configuration.
    pub fn new(config: WasapiDeviceBufferWriterConfig) -> Self {
        Self {
            config,
            state: WasapiDeviceBufferWriterState::new(),
            render_client: None,
        }
    }

    /// Creates a new writer with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(WasapiDeviceBufferWriterConfig::default())
    }

    /// Creates a new writer with a render client boundary.
    pub fn with_render_client(
        config: WasapiDeviceBufferWriterConfig,
        render_client: Box<dyn RenderClientBoundary>,
    ) -> Self {
        Self {
            config,
            state: WasapiDeviceBufferWriterState::new(),
            render_client: Some(render_client),
        }
    }
}

// ── Accessors ─────────────────────────────────────────────────────────────

impl WasapiDeviceBufferWriter {
    /// Returns a reference to the writer configuration.
    pub fn config(&self) -> &WasapiDeviceBufferWriterConfig {
        &self.config
    }

    /// Returns a reference to the internal state.
    pub fn internal_state(&self) -> &WasapiDeviceBufferWriterState {
        &self.state
    }

    /// Returns a mutable reference to the internal state.
    pub fn internal_state_mut(&mut self) -> &mut WasapiDeviceBufferWriterState {
        &mut self.state
    }

    /// Returns true if this writer has a render client boundary.
    pub fn has_render_client(&self) -> bool {
        self.render_client.is_some()
    }

    /// Sets the render client boundary.
    pub fn set_render_client(&mut self, render_client: Box<dyn RenderClientBoundary>) {
        self.render_client = Some(render_client);
    }

    /// Removes and returns the render client boundary, if any.
    pub fn take_render_client(&mut self) -> Option<Box<dyn RenderClientBoundary>> {
        self.render_client.take()
    }

    /// Returns a reference to the render client boundary, if any.
    pub fn render_client(&self) -> Option<&dyn RenderClientBoundary> {
        self.render_client.as_deref()
    }
}

// ── DeviceBufferWriter trait implementation ────────────────────────────────

impl DeviceBufferWriter for WasapiDeviceBufferWriter {
    fn process_request(&mut self, request: &WriteRequest) -> Result<WriteResult, WriteError> {
        if self.state.is_closed {
            return Err(WriteError::DeviceClosed);
        }

        match request {
            WriteRequest::WritePacket {
                frame_count,
                sample_rate,
                channel_count,
            } => self.process_write_packet(*frame_count, *sample_rate, *channel_count),
            WriteRequest::Flush => {
                self.state.buffer_fill_frames = 0;
                self.state.flush_count += 1;
                self.state.consecutive_would_blocks = 0;
                self.state.update_lifecycle(self.config.capacity_frames);
                let result = WriteResult::Noop;
                self.state.last_result = result.clone();
                Ok(result)
            }
            WriteRequest::Close => {
                self.state.is_closed = true;
                self.state.update_lifecycle(self.config.capacity_frames);
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
            lifecycle: self.state.lifecycle,
            requests_accepted: self.state.write_attempts,
            writes_completed: self.state.write_attempts - self.state.would_block_count,
            frames_written: self.state.frames_written,
            bytes_written: self.state.bytes_written,
            errors: 0,
            is_closed: self.state.is_closed,
            is_ready: !self.state.is_closed,
            buffer_fill_frames: self.state.buffer_fill_frames,
            buffer_capacity_frames: self.config.capacity_frames,
            buffer_wrap_count: self.state.wrap_count,
            would_block_count: self.state.would_block_count,
            flush_count: self.state.flush_count,
            consecutive_would_blocks: self.state.consecutive_would_blocks,
            max_consecutive_would_blocks: self.state.max_consecutive_would_blocks,
            write_streak: self.state.write_streak,
            max_write_streak: self.state.max_write_streak,
            runtime_mode: self.state.runtime_mode,
            readiness: self.state.readiness,
        }
    }

    fn cursor(&self) -> WriterCursor {
        WriterCursor {
            lifecycle: self.state.lifecycle,
            write_position: self.state.write_head,
            buffer_capacity: self.config.capacity_frames,
            buffered_frames: self.state.buffer_fill_frames,
            sample_rate: self.config.sample_rate,
            channel_count: self.config.channels,
            total_frames_written: self.state.frames_written,
            wrap_count: self.state.wrap_count,
        }
    }

    fn is_ready(&self) -> bool {
        if self.state.is_closed {
            return false;
        }
        if let Some(client) = &self.render_client {
            return client.is_ready();
        }
        true
    }

    fn is_closed(&self) -> bool {
        self.state.is_closed
    }

    fn reset(&mut self) {
        self.state = WasapiDeviceBufferWriterState::new();
        // Note: render_client is preserved across resets
    }
}
