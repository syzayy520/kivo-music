/// Errors for ring buffer operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingBufferError {
    /// Invalid format (block_align=0, channels=0, bits_per_sample=0, sample_rate_hz=0)
    InvalidFormat,
    /// Capacity must be > 0
    InvalidCapacity,
    /// Buffer is full, cannot write
    WouldBlock,
    /// Buffer is closed
    Closed,
    /// Byte length not aligned to block_align
    FrameAlignment,
}
