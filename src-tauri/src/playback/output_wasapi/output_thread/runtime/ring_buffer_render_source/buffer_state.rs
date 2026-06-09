//! Ring buffer source state type.
//!
//! Pure data snapshot of a ring buffer render source's state.
//! No behavior, no IO, no actual buffer.

/// Snapshot of a ring buffer render source's current state.
///
/// Captures the state of a ring buffer source at a point in time.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BufferState {
    /// Total requests accepted by the source.
    pub requests_accepted: u64,
    /// Total packets provided by the source.
    pub packets_provided: u64,
    /// Total frames read from the source.
    pub frames_read: u64,
    /// Total bytes read from the source.
    pub bytes_read: u64,
    /// Total errors encountered.
    pub errors: u64,
    /// Whether the source is fully exhausted.
    pub is_exhausted: bool,
    /// Whether the source is ready to serve requests.
    pub is_ready: bool,
    /// Number of packets currently in the buffer.
    pub packets_in_buffer: u64,
    /// Current buffer fill level in frames.
    pub buffer_fill_frames: u64,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u64,
    /// Number of buffer wraps that have occurred.
    pub buffer_wrap_count: u64,
    /// Total underrun events (read when buffer empty).
    pub underrun_count: u64,
    /// Total overrun events (write when buffer full).
    pub overrun_count: u64,
}

impl BufferState {
    /// Returns true if the buffer is empty.
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer_fill_frames == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_buffer_full(&self) -> bool {
        self.buffer_capacity_frames > 0 && self.buffer_fill_frames >= self.buffer_capacity_frames
    }

    /// Returns the buffer fill percentage (0-100).
    pub fn buffer_fill_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.buffer_fill_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns the average packet size in frames.
    pub fn average_packet_size(&self) -> u64 {
        if self.packets_provided == 0 {
            0
        } else {
            self.frames_read / self.packets_provided
        }
    }

    /// Returns true if any underruns have occurred.
    pub fn has_underruns(&self) -> bool {
        self.underrun_count > 0
    }

    /// Returns true if any overruns have occurred.
    pub fn has_overruns(&self) -> bool {
        self.overrun_count > 0
    }
}
