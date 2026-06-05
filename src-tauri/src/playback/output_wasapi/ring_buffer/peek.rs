//! Non-destructive peek and explicit consume for RingBuffer.
//!
//! Provides `peek_frames()` and `consume_frames()` methods that allow
//! safe drain-to-render buffer workflows without data loss risk.

use super::buffer::RingBuffer;
use super::errors::RingBufferError;

impl RingBuffer {
    /// Non-destructive peek of frames from the buffer.
    ///
    /// Copies available frames into `out` without moving `read_frame`
    /// or reducing `available_frames`. Does NOT fill silence for
    /// missing frames - only copies real data.
    ///
    /// Returns the number of frames actually peeked.
    ///
    /// Behavior:
    /// - If `out.len()` is not a multiple of `block_align`, returns `FrameAlignment` error.
    /// - If `out.len() == 0`, returns `Ok(0)`.
    /// - If buffer is empty, returns `Ok(0)` without modifying `out`.
    /// - If buffer is closed and empty, returns `Ok(0)` (consistent with `read_frames_or_silence`).
    /// - Supports wrap-around: data spanning ring boundary is copied in correct order.
    /// - Does not modify stats.
    #[allow(dead_code)] // temporary until P0-074D drain wiring
    pub(crate) fn peek_frames(&self, out: &mut [u8]) -> Result<u32, RingBufferError> {
        if !out.len().is_multiple_of(self.block_align) {
            return Err(RingBufferError::FrameAlignment);
        }
        let requested_frames = (out.len() / self.block_align) as u32;
        if requested_frames == 0 {
            return Ok(0);
        }
        if self.available_frames == 0 {
            return Ok(0);
        }

        let available = self.available_frames;
        let frames_to_peek = requested_frames.min(available);
        let cap = self.capacity_frames as usize;
        let ba = self.block_align;

        // Copy frames with wrap-around (read-only, no state changes)
        let mut dst_offset = 0;
        let mut remaining = frames_to_peek as usize;
        let mut temp_read_frame = self.read_frame;
        while remaining > 0 {
            let src_idx = (temp_read_frame as usize) % cap;
            let chunk = (cap - src_idx).min(remaining);
            let src_start = src_idx * ba;
            let src_end = src_start + chunk * ba;
            out[dst_offset..dst_offset + chunk * ba]
                .copy_from_slice(&self.data[src_start..src_end]);
            temp_read_frame = (temp_read_frame + chunk as u32) % self.capacity_frames;
            dst_offset += chunk * ba;
            remaining -= chunk;
        }

        Ok(frames_to_peek)
    }

    /// Explicit consume of frames from the buffer.
    ///
    /// Advances `read_frame` and reduces `available_frames` by the
    /// requested number of frames. Does NOT copy data.
    ///
    /// Returns the number of frames actually consumed.
    ///
    /// Behavior:
    /// - If `frames == 0`, returns `Ok(0)` without modifying state.
    /// - If `frames > available_frames`, returns `NotEnoughFrames` error.
    /// - If buffer is closed and empty, returns `Ok(0)` (consistent with `read_frames_or_silence`).
    /// - Updates `stats.total_frames_read`.
    /// - Does not perform partial consume.
    #[allow(dead_code)] // temporary until P0-074D drain wiring
    pub(crate) fn consume_frames(&mut self, frames: u32) -> Result<u32, RingBufferError> {
        if frames == 0 {
            return Ok(0);
        }
        if self.available_frames == 0 {
            if self.closed {
                return Err(RingBufferError::Closed);
            }
            return Ok(0);
        }
        if frames > self.available_frames {
            return Err(RingBufferError::NotEnoughFrames {
                requested: frames,
                available: self.available_frames,
            });
        }

        // Advance read_frame and reduce available_frames
        self.read_frame = (self.read_frame + frames) % self.capacity_frames;
        self.available_frames -= frames;
        self.stats.total_frames_read += frames as u64;

        Ok(frames)
    }
}
