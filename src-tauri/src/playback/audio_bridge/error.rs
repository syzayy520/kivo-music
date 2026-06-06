use crate::playback::output_wasapi::ring_buffer::RingBufferError;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceToRingBufferBridgeError {
    InvalidChannels,
    InvalidSampleRate,
    InvalidSampleCount,
    UnsupportedSampleFormat,
    FormatMismatch,
    ByteLengthOverflow,
    RingBufferWriteFailed(RingBufferError),
    SourceClosed,
    BufferFull,
    PartialWrite { requested: u32, written: u32 },
}
