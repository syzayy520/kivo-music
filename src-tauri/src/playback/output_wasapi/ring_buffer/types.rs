/// Format binding for ring buffer.
/// Frame-oriented: all capacity and read/write operations use frame counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingBufferFormat {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
}

/// Statistics for ring buffer operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct RingBufferStats {
    pub underrun_count: u64,
    pub overrun_count: u64,
    pub total_frames_written: u64,
    pub total_frames_read: u64,
    pub total_silence_frames_filled: u64,
}
