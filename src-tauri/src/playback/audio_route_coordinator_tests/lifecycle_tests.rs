use crate::playback::audio_route_coordinator::AudioRouteCoordinatorError;

use super::fixtures::{chunk, coordinator};

#[test]
fn reset_route_clears_pending_frames_and_counters() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    coordinator
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    let report = coordinator
        .reset_route()
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.initialized);
    assert!(report.route_initialized);
    assert_eq!(report.pending_frames, 0);
    assert_eq!(report.feed_count, 0);
    assert_eq!(report.total_accepted_frames, 0);
    Ok(())
}

#[test]
fn close_marks_coordinator_closed() -> Result<(), String> {
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator.close();

    assert!(report.closed);
    assert!(coordinator.is_closed());
    Ok(())
}

#[test]
fn feed_after_close_returns_closed() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    coordinator.close();

    assert_eq!(
        coordinator.feed_pcm_source_chunk(chunk(&samples)),
        Err(AudioRouteCoordinatorError::Closed)
    );
    assert!(coordinator.report().closed);
    Ok(())
}
