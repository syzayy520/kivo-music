use super::super::capabilities::PlaybackCapabilities;
use super::super::native_pipeline::NativePipeline;
#[cfg(test)]
use super::super::native_pipeline::NativePipelineState;
use super::super::state::PlaybackState;
use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::native_playback::KivoNativePlayback;
use tap_diagnostic::TapDiagnosticState;

mod control;
mod engine;
mod load;
mod tap_diagnostic;

#[cfg(test)]
mod tap_diagnostic_tests;

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
    tap_diagnostic: TapDiagnosticState,
}

impl KivoNativeEngine {
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
            playback: KivoNativePlayback::new(),
            pipeline: NativePipeline::new(),
            state: PlaybackState::default(),
            tap_diagnostic: tap_diagnostic::state_from_config(None),
        }
    }

    #[cfg(test)]
    pub(in crate::playback) fn new_with_tap_diagnostic_policy(
        config: super::super::native_pipeline_route_tap_diagnostic_policy::NativeTapDiagnosticConfig,
    ) -> Self {
        let mut engine = Self::new();
        engine.tap_diagnostic = tap_diagnostic::state_from_config(Some(config));
        engine
    }

    #[cfg(test)]
    pub(in crate::playback) fn pipeline_state(&self) -> NativePipelineState {
        self.pipeline.snapshot()
    }
}

impl Default for KivoNativeEngine {
    fn default() -> Self {
        Self::new()
    }
}
