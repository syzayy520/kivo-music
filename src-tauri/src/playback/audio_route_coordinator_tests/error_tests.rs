use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_route::AudioRouteError;
use crate::playback::audio_route_coordinator::AudioRouteCoordinatorError;

use super::fixtures::{chunk, coordinator, mismatch_chunk};

#[test]
fn full_buffer_error_maps_to_route_error_and_backpressure() -> Result<(), String> {
    let first = [0.0, 0.1];
    let second = [0.2, 0.3];
    let mut coordinator = coordinator(1).map_err(|error| format!("{error:?}"))?;
    coordinator
        .feed_pcm_source_chunk(chunk(&first))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        coordinator.feed_pcm_source_chunk(chunk(&second)),
        Err(AudioRouteCoordinatorError::Route(AudioRouteError::Bridge(
            SourceToRingBufferBridgeError::BufferFull
        )))
    );
    assert_eq!(coordinator.report().backpressure_count, 1);
    Ok(())
}

#[test]
fn format_mismatch_maps_to_route_error() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        coordinator.feed_pcm_source_chunk(mismatch_chunk(&samples)),
        Err(AudioRouteCoordinatorError::Route(
            AudioRouteError::FormatMismatch
        ))
    );
    Ok(())
}
