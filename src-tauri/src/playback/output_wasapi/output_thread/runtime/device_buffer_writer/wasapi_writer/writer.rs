//! WASAPI device buffer writer runtime placeholder.
//!
//! Simulated buffer behavior for WASAPI device buffer writer.
//! No real WASAPI calls, no COM objects, no audio data processing.
//! Simulates buffer fill / WouldBlock / Flush / Close without real device.

use super::super::{
    frame_bytes, DeviceBufferWriter, WriteError, WriteRequest, WriteResult, WriterCursor,
    WriterState,
};
use super::{WasapiDeviceBufferWriterConfig, WasapiDeviceBufferWriterState};

/// WASAPI device buffer writer runtime placeholder.
///
/// Simulates device buffer write behavior without real WASAPI calls.
/// Supports WritePacket (with simulated fill/WouldBlock), Flush, Close, Noop.
/// No real GetBuffer/ReleaseBuffer/IAudioRenderClient.
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

    /// Returns a mutable reference to the internal state.
    pub fn internal_state_mut(&mut self) -> &mut WasapiDeviceBufferWriterState {
        &mut self.state
    }

    /// Validates WritePacket request parameters.
    fn validate_write_packet(
        &self,
        frame_count: u64,
        sample_rate: u32,
        channel_count: u16,
    ) -> Result<(), WriteError> {
        if frame_count == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "frame_count must be > 0".to_string(),
            });
        }
        if channel_count == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "channel_count must be > 0".to_string(),
            });
        }
        if sample_rate == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "sample_rate must be > 0".to_string(),
            });
        }
        Ok(())
    }

    /// Processes a WritePacket request with simulated buffer behavior.
    fn process_write_packet(
        &mut self,
        frame_count: u64,
        sample_rate: u32,
        channel_count: u16,
    ) -> Result<WriteResult, WriteError> {
        self.validate_write_packet(frame_count, sample_rate, channel_count)?;
        self.state.write_attempts += 1;

        // Check simulated buffer capacity
        let free_frames = self
            .config
            .capacity_frames
            .saturating_sub(self.state.buffer_fill_frames);
        if frame_count > free_frames {
            self.state.would_block_count += 1;
            self.state.consecutive_would_blocks += 1;
            if self.state.consecutive_would_blocks > self.state.max_consecutive_would_blocks {
                self.state.max_consecutive_would_blocks = self.state.consecutive_would_blocks;
            }
            // Reset write streak on WouldBlock
            self.state.write_streak = 0;
            let result = WriteResult::WouldBlock;
            self.state.last_result = result.clone();
            return Ok(result);
        }

        let bytes_written = frame_bytes::f32_packet_byte_count(frame_count, channel_count)
            .ok_or_else(|| WriteError::Internal {
                description: "device buffer packet byte count overflow".to_string(),
            })?;

        // Reset consecutive would-block counter on successful write
        self.state.consecutive_would_blocks = 0;

        // Track write streak
        self.state.write_streak += 1;
        if self.state.write_streak > self.state.max_write_streak {
            self.state.max_write_streak = self.state.write_streak;
        }

        self.state.buffer_fill_frames += frame_count;
        self.state.frames_written += frame_count;
        self.state.bytes_written += bytes_written;
        self.state.update_lifecycle(self.config.capacity_frames);

        // Update circular buffer position
        let new_write_head = self.state.write_head + frame_count;
        if new_write_head >= self.config.capacity_frames {
            self.state.wrap_count += new_write_head / self.config.capacity_frames;
            self.state.write_head = new_write_head % self.config.capacity_frames;
        } else {
            self.state.write_head = new_write_head;
        }

        let result = WriteResult::Written {
            frames_written: frame_count,
            bytes_written,
        };
        self.state.last_result = result.clone();
        Ok(result)
    }
}

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
                // Reset stall state on flush (buffer is now empty)
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
        !self.state.is_closed
    }

    fn is_closed(&self) -> bool {
        self.state.is_closed
    }

    fn reset(&mut self) {
        self.state = WasapiDeviceBufferWriterState::new();
    }
}
