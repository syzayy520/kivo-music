use crate::playback::audio_route_pipeline_tap::{
    AudioRoutePipelineTap, AudioRoutePipelineTapConfig, AudioRoutePipelineTapError,
};
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

pub(super) fn stream_info() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

pub(super) fn tap_config(capacity_frames: u32) -> AudioRoutePipelineTapConfig {
    AudioRoutePipelineTapConfig {
        integration: crate::playback::audio_route_integration::AudioRouteIntegrationConfig {
            coordinator: crate::playback::audio_route_coordinator::AudioRouteCoordinatorConfig {
                route: crate::playback::audio_route::AudioRouteConfig {
                    stream: stream_info(),
                    capacity_frames,
                },
            },
        },
    }
}

pub(super) fn tap(
    capacity_frames: u32,
) -> Result<AudioRoutePipelineTap, AudioRoutePipelineTapError> {
    AudioRoutePipelineTap::new(tap_config(capacity_frames))
}

pub(super) fn output_frame(samples: Vec<f32>) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream_info(),
        position_ms: 100,
        samples,
    }
}
