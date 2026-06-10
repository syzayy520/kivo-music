//! Write packet processing for WasapiDeviceBufferWriter.
//!
//! Contains the routing logic for WritePacket requests: when a render client
//! is present, delegates to it; otherwise uses simulated buffer behavior.
//! Both paths update local tracking state (frames_written, bytes_written,
//! write_streak, would_block_count, circular buffer position).

use super::super::frame_bytes;
use super::super::{WriteError, WriteResult};
use super::WasapiDeviceBufferWriter;
use crate::playback::output_wasapi::render_client_boundary::{
    BufferAcquireRequest, BufferReleaseRequest, RenderClientFailure,
};

impl WasapiDeviceBufferWriter {
    /// Processes a WritePacket request.
    ///
    /// Routes to delegated path (render client present) or simulated path.
    /// Validates parameters and increments write_attempts before routing.
    pub(super) fn process_write_packet(
        &mut self,
        frame_count: u64,
        sample_rate: u32,
        channel_count: u16,
    ) -> Result<WriteResult, WriteError> {
        self.validate_write_packet(frame_count, sample_rate, channel_count)?;
        self.state.write_attempts += 1;

        if self.render_client.is_some() {
            return self.process_write_packet_delegated(frame_count, sample_rate, channel_count);
        }

        self.process_write_packet_simulated(frame_count, sample_rate, channel_count)
    }

    /// Processes a WritePacket by delegating to the attached RenderClientBoundary.
    ///
    /// Acquires a buffer slot from the render client, updates local tracking state,
    /// and releases the buffer. BufferTooSmall from the render client is treated
    /// as WouldBlock (recoverable).
    fn process_write_packet_delegated(
        &mut self,
        frame_count: u64,
        sample_rate: u32,
        channel_count: u16,
    ) -> Result<WriteResult, WriteError> {
        let frame_count_u32: u32 = frame_count.try_into().map_err(|_| WriteError::Internal {
            description: "frame_count overflow for render client u32".to_string(),
        })?;

        let acquire_request = BufferAcquireRequest::new(
            frame_count_u32,
            sample_rate,
            channel_count,
            4, // bytes per sample for f32
        );

        // Safe: caller checks render_client.is_some()
        let client = self.render_client.as_mut().unwrap();

        match client.acquire_buffer(&acquire_request) {
            Ok(_acquire_result) => {
                let release_request = BufferReleaseRequest::new(frame_count_u32, 0);
                if let Err(failure) = client.release_buffer(&release_request) {
                    return Err(Self::map_render_client_failure(failure));
                }

                let bytes_written = frame_bytes::f32_packet_byte_count(frame_count, channel_count)
                    .ok_or_else(|| WriteError::Internal {
                        description: "device buffer packet byte count overflow".to_string(),
                    })?;

                self.apply_successful_write(frame_count, bytes_written);
                Ok(WriteResult::Written {
                    frames_written: frame_count,
                    bytes_written,
                })
            }
            Err(RenderClientFailure::BufferTooSmall { .. }) => {
                let result = self.apply_would_block();
                Ok(result)
            }
            Err(failure) => Err(Self::map_render_client_failure(failure)),
        }
    }

    /// Processes a WritePacket using simulated buffer capacity (no render client).
    ///
    /// Checks simulated capacity against config.capacity_frames and
    /// current buffer_fill_frames.
    fn process_write_packet_simulated(
        &mut self,
        frame_count: u64,
        _sample_rate: u32,
        channel_count: u16,
    ) -> Result<WriteResult, WriteError> {
        let free_frames = self
            .config
            .capacity_frames
            .saturating_sub(self.state.buffer_fill_frames);
        if frame_count > free_frames {
            let result = self.apply_would_block();
            return Ok(result);
        }

        let bytes_written = frame_bytes::f32_packet_byte_count(frame_count, channel_count)
            .ok_or_else(|| WriteError::Internal {
                description: "device buffer packet byte count overflow".to_string(),
            })?;

        self.apply_successful_write(frame_count, bytes_written);
        Ok(WriteResult::Written {
            frames_written: frame_count,
            bytes_written,
        })
    }

    /// Applies state updates after a successful write.
    ///
    /// Updates: consecutive_would_blocks (reset), write_streak, buffer_fill_frames,
    /// frames_written, bytes_written, lifecycle, circular buffer position, last_result.
    fn apply_successful_write(&mut self, frame_count: u64, bytes_written: u64) {
        self.state.consecutive_would_blocks = 0;
        self.state.write_streak += 1;
        if self.state.write_streak > self.state.max_write_streak {
            self.state.max_write_streak = self.state.write_streak;
        }

        self.state.buffer_fill_frames += frame_count;
        self.state.frames_written += frame_count;
        self.state.bytes_written += bytes_written;
        self.state.update_lifecycle(self.config.capacity_frames);

        let new_write_head = self.state.write_head + frame_count;
        if new_write_head >= self.config.capacity_frames {
            self.state.wrap_count += new_write_head / self.config.capacity_frames;
            self.state.write_head = new_write_head % self.config.capacity_frames;
        } else {
            self.state.write_head = new_write_head;
        }

        self.state.last_result = WriteResult::Written {
            frames_written: frame_count,
            bytes_written,
        };
    }

    /// Applies state updates for a WouldBlock result.
    ///
    /// Updates: would_block_count, consecutive_would_blocks, max_consecutive_would_blocks,
    /// write_streak (reset), last_result.
    fn apply_would_block(&mut self) -> WriteResult {
        self.state.would_block_count += 1;
        self.state.consecutive_would_blocks += 1;
        if self.state.consecutive_would_blocks > self.state.max_consecutive_would_blocks {
            self.state.max_consecutive_would_blocks = self.state.consecutive_would_blocks;
        }
        self.state.write_streak = 0;
        let result = WriteResult::WouldBlock;
        self.state.last_result = result.clone();
        result
    }
}
