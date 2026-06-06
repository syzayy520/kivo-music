use crate::playback::audio_bridge::{
    from_decoded_frame, from_output_frame, write_pcm_source_chunk_to_ring_buffer, PcmSourceFormat,
};
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo, DecodedAudioFrame};
use crate::playback::output::AudioOutputFrame;

use super::fixtures::{closed_chunk, float_stream, ring_buffer};

#[test]
fn source_closed_empty_samples_returns_report() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let report = write_pcm_source_chunk_to_ring_buffer(closed_chunk(&samples), &mut buffer)
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.source_closed);
    assert_eq!(report.requested_frames, 0);
    assert_eq!(report.accepted_frames, 0);
    assert_eq!(report.bytes_written, 0);
    assert!(report.format_validated);
    Ok(())
}

#[test]
fn decoded_frame_adapter_borrows_samples() {
    let frame = DecodedAudioFrame {
        stream: float_stream(),
        position_ms: 42,
        samples: vec![0.0, 0.5],
    };
    let chunk = from_decoded_frame(&frame);

    assert_eq!(chunk.position_ms, 42);
    assert_eq!(chunk.samples, frame.samples.as_slice());
    assert!(!chunk.source_closed);
}

#[test]
fn output_frame_adapter_borrows_samples() {
    let frame = AudioOutputFrame {
        stream: float_stream(),
        position_ms: 64,
        samples: vec![1.0, -1.0],
    };
    let chunk = from_output_frame(&frame);

    assert_eq!(chunk.position_ms, 64);
    assert_eq!(chunk.samples, frame.samples.as_slice());
    assert!(!chunk.source_closed);
}

#[test]
fn source_format_reflects_stream_fields() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 6,
        sample_format: AudioSampleFormat::Float32,
    };
    let source_format = PcmSourceFormat::from(&stream);

    assert_eq!(source_format.sample_rate_hz, 48_000);
    assert_eq!(source_format.channels, 6);
    assert!(matches!(
        source_format.sample_format,
        AudioSampleFormat::Float32
    ));
}
