use crate::playback::audio_bridge::write_pcm_source_chunk_to_ring_buffer;

use super::fixtures::{chunk, closed_chunk, read_bytes, ring_buffer};

fn expected_le_bytes(samples: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}

#[test]
fn source_to_ring_buffer_writes_float32_little_endian_bytes() -> Result<(), String> {
    let samples = [1.0, -2.0, 0.5, -0.25];
    let mut buffer = ring_buffer(4)?;
    let report = write_pcm_source_chunk_to_ring_buffer(chunk(&samples), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.requested_frames, 2);
    assert_eq!(report.accepted_frames, 2);
    assert_eq!(report.bytes_written, 16);
    assert_eq!(read_bytes(&mut buffer, 2)?, expected_le_bytes(&samples));
    Ok(())
}

#[test]
fn empty_samples_return_noop_report() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let report = write_pcm_source_chunk_to_ring_buffer(chunk(&samples), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.requested_frames, 0);
    assert_eq!(report.accepted_frames, 0);
    assert_eq!(report.rejected_frames, 0);
    assert_eq!(report.bytes_written, 0);
    assert_eq!(buffer.available_frames(), 0);
    assert!(report.format_validated);
    Ok(())
}

#[test]
fn source_closed_non_empty_chunk_is_written() -> Result<(), String> {
    let samples = [0.25, 0.5];
    let mut buffer = ring_buffer(4)?;
    let report = write_pcm_source_chunk_to_ring_buffer(closed_chunk(&samples), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.source_closed);
    assert_eq!(report.accepted_frames, 1);
    assert_eq!(buffer.available_frames(), 1);
    Ok(())
}
