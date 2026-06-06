use crate::playback::audio_bridge::{from_parts, PcmSourceChunk};
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output_wasapi::ring_buffer::{RingBuffer, RingBufferFormat};

pub(super) fn float_stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 44_100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

pub(super) fn stream_with(
    sample_rate_hz: u32,
    channels: u16,
    sample_format: AudioSampleFormat,
) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format,
    }
}

pub(super) fn ring_format(sample_rate_hz: u32, channels: u16) -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz,
        channels,
        bits_per_sample: 32,
        block_align: channels * 4,
    }
}

pub(super) fn ring_buffer(capacity_frames: u32) -> Result<RingBuffer, String> {
    RingBuffer::new(ring_format(44_100, 2), capacity_frames).map_err(|error| format!("{error:?}"))
}

pub(super) fn ring_buffer_with(
    format: RingBufferFormat,
    capacity_frames: u32,
) -> Result<RingBuffer, String> {
    RingBuffer::new(format, capacity_frames).map_err(|error| format!("{error:?}"))
}

pub(super) fn chunk(samples: &[f32]) -> PcmSourceChunk<'_> {
    from_parts(float_stream(), 123, samples, false)
}

pub(super) fn closed_chunk(samples: &[f32]) -> PcmSourceChunk<'_> {
    from_parts(float_stream(), 123, samples, true)
}

pub(super) fn read_bytes(buffer: &mut RingBuffer, frames: u32) -> Result<Vec<u8>, String> {
    let block_align = usize::from(buffer.format().block_align);
    let mut out =
        vec![0u8; usize::try_from(frames).map_err(|error| format!("{error:?}"))? * block_align];
    let read = buffer
        .read_frames_or_silence(&mut out)
        .map_err(|error| format!("{error:?}"))?;
    if read != frames {
        return Err(format!("read {read} frames, wanted {frames}"));
    }
    Ok(out)
}
