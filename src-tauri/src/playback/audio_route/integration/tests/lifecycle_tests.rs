use crate::playback::audio_route_integration::AudioRouteIntegrationError;

use super::fixtures::{chunk, integration};

#[test]
fn reset_route_clears_pending_frames_and_input_count() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3];
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    integration
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    let report = integration
        .reset_route()
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.initialized);
    assert!(report.coordinator_initialized);
    assert_eq!(report.pending_frames, 0);
    assert_eq!(report.input_count, 0);
    assert_eq!(report.total_accepted_frames, 0);
    Ok(())
}

#[test]
fn close_marks_integration_closed() -> Result<(), String> {
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    let report = integration.close();

    assert!(report.closed);
    assert!(integration.is_closed());
    Ok(())
}

#[test]
fn feed_after_close_returns_closed() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    integration.close();

    assert_eq!(
        integration.feed_pcm_source_chunk(chunk(&samples)),
        Err(AudioRouteIntegrationError::Closed)
    );
    assert!(integration.report().closed);
    Ok(())
}
