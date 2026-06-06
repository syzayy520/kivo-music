use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_route::AudioRouteError;

use super::fixtures::{chunk, owner};

#[test]
fn buffer_full_is_mapped_without_panic() -> Result<(), String> {
    let first = [0.0, 0.1];
    let second = [0.2, 0.3];
    let mut owner = owner(1).map_err(|error| format!("{error:?}"))?;
    owner
        .feed_pcm_source_chunk(chunk(&first))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        owner.feed_pcm_source_chunk(chunk(&second)),
        Err(AudioRouteError::Bridge(
            SourceToRingBufferBridgeError::BufferFull
        ))
    );
    Ok(())
}

#[test]
fn partial_write_updates_report_totals() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3];
    let mut owner = owner(1).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.last_requested_frames, 2);
    assert_eq!(report.last_accepted_frames, 1);
    assert_eq!(report.last_rejected_frames, 1);
    assert_eq!(report.total_requested_frames, 2);
    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.total_rejected_frames, 1);
    assert!(report.last_ring_buffer_full);
    assert!(report.last_partial_write);
    Ok(())
}
