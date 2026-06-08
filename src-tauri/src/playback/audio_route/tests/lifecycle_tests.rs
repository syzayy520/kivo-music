use crate::playback::audio_route::AudioRouteError;

use super::fixtures::{chunk, owner};

#[test]
fn reset_clears_pending_frames_and_report_totals() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    owner
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    owner.reset();

    let report = owner.report();
    assert_eq!(owner.pending_frames(), 0);
    assert_eq!(report.total_requested_frames, 0);
    assert_eq!(report.total_accepted_frames, 0);
    assert_eq!(report.last_requested_frames, 0);
    assert!(report.initialized);
    Ok(())
}

#[test]
fn close_marks_owner_closed() -> Result<(), String> {
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    owner.close();

    assert!(owner.is_closed());
    Ok(())
}

#[test]
fn feed_after_close_returns_closed() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    owner.close();

    assert_eq!(
        owner.feed_pcm_source_chunk(chunk(&samples)),
        Err(AudioRouteError::Closed)
    );
    Ok(())
}
