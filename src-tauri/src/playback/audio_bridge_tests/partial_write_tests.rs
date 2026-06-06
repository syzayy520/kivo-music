use crate::playback::audio_bridge::write_pcm_source_chunk_to_ring_buffer;

use super::fixtures::{chunk, ring_buffer};

#[test]
fn partial_write_is_reported_not_rejected() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
    let mut buffer = ring_buffer(2)?;
    let report = write_pcm_source_chunk_to_ring_buffer(chunk(&samples), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.requested_frames, 3);
    assert_eq!(report.accepted_frames, 2);
    assert_eq!(report.rejected_frames, 1);
    assert_eq!(report.bytes_written, 16);
    assert_eq!(report.pending_frames_after_write, 2);
    assert!(report.ring_buffer_full);
    assert!(report.partial_write);
    Ok(())
}
