use super::super::capabilities::PlaybackCapabilities;
use super::super::native_pipeline::NativePipeline;
#[cfg(test)]
use super::super::native_pipeline::NativePipelineState;
use super::super::state::PlaybackState;
use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::native_playback::KivoNativePlayback;

mod control;
mod engine;
mod load;

pub fn descriptor() -> PlaybackBackendDescriptor {
    PlaybackBackendDescriptor {
        kind: PlaybackBackendKind::Native,
        name: "kivo-core-audio".to_string(),
        capabilities: PlaybackCapabilities::default(),
        is_primary: true,
    }
}

#[derive(Debug)]
pub struct KivoNativeEngine {
    descriptor: PlaybackBackendDescriptor,
    playback: KivoNativePlayback,
    pipeline: NativePipeline,
    state: PlaybackState,
}

impl KivoNativeEngine {
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
            playback: KivoNativePlayback::new(),
            pipeline: NativePipeline::new(),
            state: PlaybackState::default(),
        }
    }

    #[cfg(test)]
    pub fn pipeline_state(&self) -> NativePipelineState {
        self.pipeline.state()
    }
}

impl Default for KivoNativeEngine {
    fn default() -> Self {
        Self::new()
    }
}
