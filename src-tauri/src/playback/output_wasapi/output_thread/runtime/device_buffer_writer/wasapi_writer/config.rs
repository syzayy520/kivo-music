//! WASAPI device buffer writer configuration.
//!
//! Pure value configuration for type scaffold.
//! No COM pointers, no device handles, no Windows API types.

/// Configuration for a WASAPI device buffer writer.
///
/// Contains only pure value fields for audio format specification.
/// No real WASAPI resources are held by this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WasapiDeviceBufferWriterConfig {
    /// Buffer capacity in frames.
    pub capacity_frames: u64,
    /// Number of audio channels.
    pub channels: u16,
    /// Sample rate in Hz.
    pub sample_rate: u32,
}

impl WasapiDeviceBufferWriterConfig {
    /// Creates a new configuration with the given parameters.
    pub fn new(capacity_frames: u64, channels: u16, sample_rate: u32) -> Self {
        Self {
            capacity_frames,
            channels,
            sample_rate,
        }
    }

    /// Creates a default configuration (44100 Hz, stereo, 1024 frames).
    pub fn default_config() -> Self {
        Self::new(1024, 2, 44100)
    }

    /// Returns the buffer capacity in frames.
    pub fn capacity_frames(&self) -> u64 {
        self.capacity_frames
    }

    /// Returns the number of audio channels.
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Returns the sample rate in Hz.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl Default for WasapiDeviceBufferWriterConfig {
    fn default() -> Self {
        Self::default_config()
    }
}
