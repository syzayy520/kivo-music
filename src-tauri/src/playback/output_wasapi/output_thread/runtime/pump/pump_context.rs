//! Pump context.
//!
//! Pure-memory input for a single runtime pump tick.

use crate::playback::output_wasapi::output_thread::command::thread_command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;

/// Configuration for a pump tick.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PumpConfig {
    pub frame_count: u64,
    pub sample_rate: u32,
    pub channel_count: u16,
    pub max_idle_steps: u64,
}

impl Default for PumpConfig {
    fn default() -> Self {
        Self {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
            max_idle_steps: 100,
        }
    }
}

/// Input context for a single runtime pump tick.
#[derive(Debug, Clone)]
pub struct PumpContext {
    pub state: LoopState,
    pub command: Option<ThreadCommand>,
    pub config: PumpConfig,
}

impl PumpContext {
    /// Create a new pump context.
    pub fn new(state: LoopState, command: Option<ThreadCommand>, config: PumpConfig) -> Self {
        Self {
            state,
            command,
            config,
        }
    }

    /// Create a pump context with default config.
    pub fn with_defaults(state: LoopState, command: Option<ThreadCommand>) -> Self {
        Self {
            state,
            command,
            config: PumpConfig::default(),
        }
    }
}
