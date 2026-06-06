use super::super::render_loop::RenderSilenceLoopConfig;
use super::super::render_once::RenderSilenceOnceConfig;
use super::super::render_padding_loop::RenderPaddingLoopConfig;
use super::super::render_ring_buffer_boundary::RenderRingBufferBoundaryConfig;
use super::super::thread::RealOutputThreadSpawnConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RealOutputThreadStageConfigs {
    pub render_once: RenderSilenceOnceConfig,
    pub render_loop: RenderSilenceLoopConfig,
    pub render_padding_loop: RenderPaddingLoopConfig,
    pub render_ring_buffer_boundary: RenderRingBufferBoundaryConfig,
}

impl RealOutputThreadStageConfigs {
    pub(crate) fn from_spawn_config(config: RealOutputThreadSpawnConfig) -> Self {
        Self {
            render_once: RenderSilenceOnceConfig {
                enabled: config.render_silence_once_after_open,
                frames: config.render_silence_once_frames,
            },
            render_loop: RenderSilenceLoopConfig {
                enabled: config.render_silence_loop_after_start,
                iterations: config.render_silence_loop_iterations,
                frames_per_write: config.render_silence_loop_frames_per_write,
            },
            render_padding_loop: RenderPaddingLoopConfig {
                enabled: config.render_padding_loop_after_start,
                iterations: config.render_padding_loop_iterations,
                max_frames_per_write: config.render_padding_loop_max_frames_per_write,
            },
            render_ring_buffer_boundary: RenderRingBufferBoundaryConfig {
                enabled: config.render_ring_buffer_boundary_after_padding_loop,
                iterations: config.render_ring_buffer_boundary_iterations,
                max_frames_per_write: config.render_ring_buffer_boundary_max_frames_per_write,
                synthetic_zero_seed_frames: config
                    .render_ring_buffer_boundary_synthetic_zero_seed_frames,
            },
        }
    }
}
