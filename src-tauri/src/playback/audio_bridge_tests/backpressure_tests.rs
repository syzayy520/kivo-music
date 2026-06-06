use crate::playback::audio_bridge::{
    write_pcm_source_chunk_to_ring_buffer, SourceToRingBufferBridgeError,
};

use super::fixtures::{chunk, ring_buffer};

#[test]
fn full_ring_buffer_returns_buffer_full() -> Result<(), String> {
    let first = [0.0, 0.0];
    let second = [1.0, 1.0];
    let mut buffer = ring_buffer(1)?;
    write_pcm_source_chunk_to_ring_buffer(chunk(&first), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(chunk(&second), &mut buffer),
        Err(SourceToRingBufferBridgeError::BufferFull)
    );
    Ok(())
}
