use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_route::AudioRouteError;
use crate::playback::audio_route_coordinator::AudioRouteCoordinatorError;
use crate::playback::audio_route_integration::AudioRouteIntegrationError;

use super::fixtures::{chunk, integration, mismatch_chunk};

#[test]
fn full_buffer_error_maps_to_coordinator_error_and_backpressure() -> Result<(), String> {
    let first = [0.0, 0.1];
    let second = [0.2, 0.3];
    let mut integration = integration(1).map_err(|error| format!("{error:?}"))?;
    integration
        .feed_pcm_source_chunk(chunk(&first))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        integration.feed_pcm_source_chunk(chunk(&second)),
        Err(AudioRouteIntegrationError::Coordinator(
            AudioRouteCoordinatorError::Route(AudioRouteError::Bridge(
                SourceToRingBufferBridgeError::BufferFull
            ))
        ))
    );
    assert_eq!(integration.report().backpressure_count, 1);
    Ok(())
}

#[test]
fn format_mismatch_maps_to_coordinator_error() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        integration.feed_pcm_source_chunk(mismatch_chunk(&samples)),
        Err(AudioRouteIntegrationError::Coordinator(
            AudioRouteCoordinatorError::Route(AudioRouteError::FormatMismatch)
        ))
    );
    Ok(())
}
