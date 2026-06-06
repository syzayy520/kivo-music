use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::output_wasapi::ring_buffer::RingBufferError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRouteError {
    InvalidCapacity,
    InvalidFormat,
    FormatMismatch,
    RingBufferInitFailed(RingBufferError),
    Bridge(SourceToRingBufferBridgeError),
    Closed,
}
