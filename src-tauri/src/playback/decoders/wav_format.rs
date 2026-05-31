use hound::SampleFormat;

use crate::playback::decoder::AudioSampleFormat;
use crate::playback::errors::{PlaybackError, PlaybackResult};

#[derive(Clone, Debug)]
pub enum WavSampleKind {
    Int16,
    Int24,
    Int32,
    Float32,
}

pub fn map_sample_kind(
    spec_format: SampleFormat,
    bits_per_sample: u16,
) -> PlaybackResult<(WavSampleKind, AudioSampleFormat)> {
    match (spec_format, bits_per_sample) {
        (SampleFormat::Float, 32) => Ok((WavSampleKind::Float32, AudioSampleFormat::Float32)),
        (SampleFormat::Int, 16) => Ok((WavSampleKind::Int16, AudioSampleFormat::Signed16)),
        (SampleFormat::Int, 24) => Ok((WavSampleKind::Int24, AudioSampleFormat::Signed24)),
        (SampleFormat::Int, 32) => Ok((WavSampleKind::Int32, AudioSampleFormat::Signed32)),
        _ => Err(PlaybackError::UnsupportedFormat(format!(
            "wav {spec_format:?} {bits_per_sample}-bit"
        ))),
    }
}
