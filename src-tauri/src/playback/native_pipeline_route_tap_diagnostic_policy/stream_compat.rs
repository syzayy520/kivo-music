use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

pub fn is_same_diagnostic_route_stream(left: &AudioStreamInfo, right: &AudioStreamInfo) -> bool {
    left.sample_rate_hz == right.sample_rate_hz
        && left.channels == right.channels
        && same_sample_format(&left.sample_format, &right.sample_format)
}

fn same_sample_format(left: &AudioSampleFormat, right: &AudioSampleFormat) -> bool {
    matches!(
        (left, right),
        (AudioSampleFormat::Float32, AudioSampleFormat::Float32)
            | (AudioSampleFormat::Signed16, AudioSampleFormat::Signed16)
            | (AudioSampleFormat::Signed24, AudioSampleFormat::Signed24)
            | (AudioSampleFormat::Signed32, AudioSampleFormat::Signed32)
    )
}
