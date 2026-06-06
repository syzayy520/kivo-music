use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProductionOutputRouteStreamFormat {
    Float32,
    Signed16,
    Signed24,
    Signed32,
}

impl From<&AudioSampleFormat> for ProductionOutputRouteStreamFormat {
    fn from(sample_format: &AudioSampleFormat) -> Self {
        match sample_format {
            AudioSampleFormat::Float32 => Self::Float32,
            AudioSampleFormat::Signed16 => Self::Signed16,
            AudioSampleFormat::Signed24 => Self::Signed24,
            AudioSampleFormat::Signed32 => Self::Signed32,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteFormatDescriptor {
    sample_rate_hz: u32,
    channels: u16,
    sample_format: ProductionOutputRouteStreamFormat,
}

impl ProductionOutputRouteFormatDescriptor {
    pub(crate) fn from_stream(stream: &AudioStreamInfo) -> Self {
        Self {
            sample_rate_hz: stream.sample_rate_hz,
            channels: stream.channels,
            sample_format: ProductionOutputRouteStreamFormat::from(&stream.sample_format),
        }
    }

    pub(crate) fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    pub(crate) fn channels(&self) -> u16 {
        self.channels
    }

    pub(crate) fn sample_format(&self) -> ProductionOutputRouteStreamFormat {
        self.sample_format
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteFormatMismatch {
    expected: ProductionOutputRouteFormatDescriptor,
    actual: ProductionOutputRouteFormatDescriptor,
}

impl ProductionOutputRouteFormatMismatch {
    pub(crate) fn new(
        expected: ProductionOutputRouteFormatDescriptor,
        actual: ProductionOutputRouteFormatDescriptor,
    ) -> Self {
        Self { expected, actual }
    }

    pub(crate) fn expected(&self) -> ProductionOutputRouteFormatDescriptor {
        self.expected
    }

    pub(crate) fn actual(&self) -> ProductionOutputRouteFormatDescriptor {
        self.actual
    }

    pub(crate) fn is_mismatch(&self) -> bool {
        self.expected != self.actual
    }
}
