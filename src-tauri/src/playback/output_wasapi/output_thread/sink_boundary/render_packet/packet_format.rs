//! Packet format type.
//!
//! Describes the audio format of a render packet.
//! Pure data — no behavior, no IO.

/// Sample format for audio data.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SampleFormat {
    /// 32-bit floating point.
    #[default]
    Float32,
    /// 16-bit signed integer.
    Int16,
    /// 24-bit signed integer (packed in 32 bits).
    Int24,
    /// 32-bit signed integer.
    Int32,
}

/// Audio format descriptor for a render packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketFormat {
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Number of audio channels.
    pub channel_count: u16,
    /// Sample format.
    pub sample_format: SampleFormat,
    /// Bits per sample.
    pub bits_per_sample: u16,
}

impl Default for PacketFormat {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channel_count: 2,
            sample_format: SampleFormat::default(),
            bits_per_sample: 32,
        }
    }
}
