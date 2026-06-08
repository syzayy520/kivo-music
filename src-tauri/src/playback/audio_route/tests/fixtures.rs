use crate::playback::audio_bridge::{from_parts, PcmSourceChunk};
use crate::playback::audio_route::{AudioRouteConfig, AudioRouteError, AudioRouteOwner};
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo, DecodedAudioFrame};
use crate::playback::output::AudioOutputFrame;

pub(super) fn float_stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 44_100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

pub(super) fn stream_with(
    sample_rate_hz: u32,
    channels: u16,
    sample_format: AudioSampleFormat,
) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format,
    }
}

pub(super) fn config(capacity_frames: u32) -> AudioRouteConfig {
    AudioRouteConfig {
        stream: float_stream(),
        capacity_frames,
    }
}

pub(super) fn owner(capacity_frames: u32) -> Result<AudioRouteOwner, AudioRouteError> {
    AudioRouteOwner::new(config(capacity_frames))
}

pub(super) fn chunk(samples: &[f32]) -> PcmSourceChunk<'_> {
    from_parts(float_stream(), 0, samples, false)
}

pub(super) fn closed_chunk(samples: &[f32]) -> PcmSourceChunk<'_> {
    from_parts(float_stream(), 0, samples, true)
}

pub(super) fn mismatch_chunk(samples: &[f32]) -> PcmSourceChunk<'_> {
    from_parts(
        stream_with(48_000, 2, AudioSampleFormat::Float32),
        0,
        samples,
        false,
    )
}

pub(super) fn decoded_frame(samples: Vec<f32>) -> DecodedAudioFrame {
    DecodedAudioFrame {
        stream: float_stream(),
        position_ms: 12,
        samples,
    }
}

pub(super) fn output_frame(samples: Vec<f32>) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: float_stream(),
        position_ms: 24,
        samples,
    }
}
