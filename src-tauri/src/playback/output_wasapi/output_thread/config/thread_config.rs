//! Thread configuration type.
//!
//! Defines the runtime configuration for the output thread.

use super::buffer_config::BufferConfig;
use super::render_loop_config::RenderLoopConfig;

/// Runtime configuration for the output thread.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadConfig {
    /// Render loop configuration.
    pub render_loop: RenderLoopConfig,
    /// Buffer configuration.
    pub buffer: BufferConfig,
    /// Target sample rate in Hz.
    pub sample_rate: u32,
    /// Number of output channels.
    pub channel_count: u32,
    /// Bits per sample (16, 24, 32).
    pub bits_per_sample: u32,
}

impl Default for ThreadConfig {
    fn default() -> Self {
        Self {
            render_loop: RenderLoopConfig::default(),
            buffer: BufferConfig::default(),
            sample_rate: 44100,
            channel_count: 2,
            bits_per_sample: 16,
        }
    }
}