use crate::playback::audio_route::{AudioRouteConfig, AudioRouteError, AudioRouteOwner};
use crate::playback::decoder::AudioSampleFormat;

use super::fixtures::{config, stream_with};

#[test]
fn owner_creates_ring_buffer_from_float32_stream() -> Result<(), String> {
    let owner = AudioRouteOwner::new(config(8)).map_err(|error| format!("{error:?}"))?;

    assert_eq!(owner.capacity_frames(), 8);
    assert_eq!(owner.pending_frames(), 0);
    assert!(owner.report().initialized);
    assert!(!owner.is_closed());
    Ok(())
}

#[test]
fn config_new_validates_and_keeps_fields() -> Result<(), String> {
    let config = AudioRouteConfig::new(stream_with(48_000, 2, AudioSampleFormat::Float32), 16)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(config.stream.sample_rate_hz, 48_000);
    assert_eq!(config.capacity_frames, 16);
    Ok(())
}

#[test]
fn invalid_capacity_is_rejected() {
    assert_eq!(
        AudioRouteOwner::new(config(0)).map(|_| ()),
        Err(AudioRouteError::InvalidCapacity)
    );
}
