use crate::playback::audio_route::format::derive_route_ring_buffer_format;
use crate::playback::audio_route::{AudioRouteConfig, AudioRouteError, AudioRouteOwner};
use crate::playback::decoder::AudioSampleFormat;

use super::fixtures::{chunk, mismatch_chunk, owner, stream_with};

#[test]
fn route_format_is_derived_from_stream() -> Result<(), String> {
    let format =
        derive_route_ring_buffer_format(&stream_with(44_100, 2, AudioSampleFormat::Float32))
            .map_err(|error| format!("{error:?}"))?;

    assert_eq!(format.sample_rate_hz, 44_100);
    assert_eq!(format.channels, 2);
    assert_eq!(format.bits_per_sample, 32);
    assert_eq!(format.block_align, 8);
    Ok(())
}

#[test]
fn invalid_channels_are_rejected() {
    let config = AudioRouteConfig {
        stream: stream_with(44_100, 0, AudioSampleFormat::Float32),
        capacity_frames: 4,
    };
    assert_eq!(
        AudioRouteOwner::new(config).map(|_| ()),
        Err(AudioRouteError::InvalidFormat)
    );
}

#[test]
fn invalid_sample_rate_is_rejected() {
    let config = AudioRouteConfig {
        stream: stream_with(0, 2, AudioSampleFormat::Float32),
        capacity_frames: 4,
    };
    assert_eq!(
        AudioRouteOwner::new(config).map(|_| ()),
        Err(AudioRouteError::InvalidFormat)
    );
}

#[test]
fn non_float32_is_rejected() {
    let config = AudioRouteConfig {
        stream: stream_with(44_100, 2, AudioSampleFormat::Signed16),
        capacity_frames: 4,
    };
    assert_eq!(
        AudioRouteOwner::new(config).map(|_| ()),
        Err(AudioRouteError::InvalidFormat)
    );
}

#[test]
fn feed_format_mismatch_returns_clear_error() -> Result<(), String> {
    let samples = [0.0, 0.0];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        owner.feed_pcm_source_chunk(mismatch_chunk(&samples)),
        Err(AudioRouteError::FormatMismatch)
    );
    assert_eq!(
        owner
            .feed_pcm_source_chunk(chunk(&samples))
            .map_err(|error| format!("{error:?}"))?
            .last_accepted_frames,
        1
    );
    Ok(())
}
