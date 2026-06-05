use super::errors::RingBufferError;
use super::types::{RingBufferFormat, RingBufferStats};

/// Bounded, frame-oriented, non-blocking ring buffer.
/// Single-threaded unit boundary; no Arc/Mutex/Condvar/channel/thread.
#[derive(Debug)]
pub struct RingBuffer {
    #[allow(dead_code)]
    format: RingBufferFormat,
    capacity_frames: u32,
    block_align: usize,
    data: Vec<u8>,
    read_frame: u32,
    write_frame: u32,
    available_frames: u32,
    closed: bool,
    stats: RingBufferStats,
}

impl RingBuffer {
    /// Create a new ring buffer with the given format and capacity.
    pub fn new(format: RingBufferFormat, capacity_frames: u32) -> Result<Self, RingBufferError> {
        if format.block_align == 0
            || format.channels == 0
            || format.bits_per_sample == 0
            || format.sample_rate_hz == 0
        {
            return Err(RingBufferError::InvalidFormat);
        }
        if capacity_frames == 0 {
            return Err(RingBufferError::InvalidCapacity);
        }
        let block_align = format.block_align as usize;
        let total_bytes = (capacity_frames as usize)
            .checked_mul(block_align)
            .ok_or(RingBufferError::InvalidCapacity)?;
        Ok(Self {
            format,
            capacity_frames,
            block_align,
            data: vec![0u8; total_bytes],
            read_frame: 0,
            write_frame: 0,
            available_frames: 0,
            closed: false,
            stats: RingBufferStats::default(),
        })
    }

    /// Total capacity in frames.
    pub fn capacity_frames(&self) -> u32 {
        self.capacity_frames
    }

    /// Frames available for reading.
    pub fn available_frames(&self) -> u32 {
        self.available_frames
    }

    /// Frames available for writing.
    pub fn free_frames(&self) -> u32 {
        self.capacity_frames - self.available_frames
    }

    /// True if no frames available for reading.
    pub fn is_empty(&self) -> bool {
        self.available_frames == 0
    }

    /// True if no frames available for writing.
    pub fn is_full(&self) -> bool {
        self.available_frames == self.capacity_frames
    }

    /// True if buffer has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Current statistics snapshot.
    pub fn stats(&self) -> RingBufferStats {
        self.stats
    }

    /// Read-only view of the buffer's format.
    #[allow(dead_code)]
    pub(crate) fn format(&self) -> RingBufferFormat {
        self.format
    }

    /// Write frames from byte slice. Length must be block_align aligned.
    /// Returns number of frames written.
    pub fn write_frames(&mut self, frames: &[u8]) -> Result<u32, RingBufferError> {
        if self.closed {
            return Err(RingBufferError::Closed);
        }
        if !frames.len().is_multiple_of(self.block_align) {
            return Err(RingBufferError::FrameAlignment);
        }
        let requested_frames = (frames.len() / self.block_align) as u32;
        if requested_frames == 0 {
            return Ok(0);
        }
        let free = self.free_frames();
        if free == 0 {
            self.stats.overrun_count += 1;
            return Err(RingBufferError::WouldBlock);
        }
        let frames_to_write = requested_frames.min(free);

        // Copy frames with wrap-around
        let cap = self.capacity_frames as usize;
        let ba = self.block_align;
        let mut src_offset = 0;
        let mut remaining = frames_to_write as usize;
        while remaining > 0 {
            let dst_idx = (self.write_frame as usize) % cap;
            let chunk = (cap - dst_idx).min(remaining);
            let dst_start = dst_idx * ba;
            let dst_end = dst_start + chunk * ba;
            self.data[dst_start..dst_end]
                .copy_from_slice(&frames[src_offset..src_offset + chunk * ba]);
            self.write_frame = (self.write_frame + chunk as u32) % self.capacity_frames;
            src_offset += chunk * ba;
            remaining -= chunk;
        }
        self.available_frames += frames_to_write;
        self.stats.total_frames_written += frames_to_write as u64;
        if frames_to_write < requested_frames {
            self.stats.overrun_count += 1;
        }
        Ok(frames_to_write)
    }

    /// Read frames into byte slice. If buffer has fewer frames than requested,
    /// remaining bytes are filled with silence (0). Returns number of real frames read.
    pub fn read_frames_or_silence(&mut self, out: &mut [u8]) -> Result<u32, RingBufferError> {
        if !out.len().is_multiple_of(self.block_align) {
            return Err(RingBufferError::FrameAlignment);
        }
        let requested_frames = (out.len() / self.block_align) as u32;
        if requested_frames == 0 {
            return Ok(0);
        }
        if self.is_empty() {
            if self.closed {
                return Err(RingBufferError::Closed);
            }
            // All silence
            out.fill(0);
            self.stats.underrun_count += 1;
            self.stats.total_silence_frames_filled += requested_frames as u64;
            self.stats.total_frames_read += 0;
            return Ok(0);
        }

        let available = self.available_frames;
        let frames_to_read = requested_frames.min(available);
        let silence_frames = requested_frames - frames_to_read;
        let bytes_to_read = (frames_to_read as usize) * self.block_align;
        let cap = self.capacity_frames as usize;
        let ba = self.block_align;

        // Read real frames with wrap-around
        let mut dst_offset = 0;
        let mut remaining = frames_to_read as usize;
        while remaining > 0 {
            let src_idx = (self.read_frame as usize) % cap;
            let chunk = (cap - src_idx).min(remaining);
            let src_start = src_idx * ba;
            let src_end = src_start + chunk * ba;
            out[dst_offset..dst_offset + chunk * ba]
                .copy_from_slice(&self.data[src_start..src_end]);
            self.read_frame = (self.read_frame + chunk as u32) % self.capacity_frames;
            dst_offset += chunk * ba;
            remaining -= chunk;
        }
        // Fill remaining with silence
        if silence_frames > 0 {
            out[bytes_to_read..].fill(0);
            self.stats.underrun_count += 1;
            self.stats.total_silence_frames_filled += silence_frames as u64;
        }

        self.available_frames -= frames_to_read;
        self.stats.total_frames_read += frames_to_read as u64;
        Ok(frames_to_read)
    }

    /// Close the buffer. After close, write_frames returns Closed.
    /// Remaining data can still be drained via read_frames_or_silence.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Reset the buffer: clear all data, reset cursors.
    /// Does not reopen closed state.
    pub fn reset(&mut self) {
        self.data.fill(0);
        self.read_frame = 0;
        self.write_frame = 0;
        self.available_frames = 0;
        // closed state preserved
    }
}
