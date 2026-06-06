use crate::playback::audio_bridge::from_parts;
use crate::playback::audio_route_integration::{
    AudioRouteIntegration, AudioRouteIntegrationConfig,
};

use super::fixtures::float_stream;

#[test]
fn existing_route_reset_clears_source_closed_and_all_counters() -> Result<(), String> {
    let stream = float_stream();
    let config = AudioRouteIntegrationConfig::new(stream.clone(), 2)
        .map_err(|error| format!("{error:?}"))?;
    let mut integration =
        AudioRouteIntegration::new(config).map_err(|error| format!("{error:?}"))?;
    let samples = [0.0, 0.1];
    let closed_chunk = from_parts(stream, 0, &samples, true);
    let before = integration
        .feed_pcm_source_chunk(closed_chunk)
        .map_err(|error| format!("{error:?}"))?;

    assert!(before.source_closed_seen);
    assert_eq!(before.input_count, 1);
    assert_eq!(before.pending_frames, 1);

    let reset = integration
        .reset_route()
        .map_err(|error| format!("{error:?}"))?;

    assert!(!reset.source_closed_seen);
    assert_eq!(reset.input_count, 0);
    assert_eq!(reset.pending_frames, 0);
    assert_eq!(reset.total_requested_frames, 0);
    assert_eq!(reset.total_accepted_frames, 0);
    assert_eq!(reset.total_rejected_frames, 0);
    assert_eq!(reset.backpressure_count, 0);
    assert_eq!(reset.partial_write_count, 0);
    Ok(())
}
