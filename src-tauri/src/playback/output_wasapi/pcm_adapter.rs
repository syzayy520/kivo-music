#![allow(dead_code)]

// This adapter intentionally lands before the future WASAPI byte-write path is
// integrated. Tests own the contract until a later ticket wires it into runtime.
const FLOAT32_BITS_PER_SAMPLE: u16 = 32;
const FLOAT32_BYTES_PER_SAMPLE: u16 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum PcmSampleFormat {
    Float32Interleaved,
    Signed16Interleaved,
    Signed24Interleaved,
    Signed32Interleaved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PcmRenderFormat {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub sample_format: PcmSampleFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PcmFrameBytes<'a> {
    pub frames: u32,
    pub bytes: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PcmAdapterError {
    ZeroChannels,
    ZeroSampleRate,
    UnsupportedSampleFormat,
    InvalidBitsPerSample,
    InvalidBlockAlign,
    SampleCountNotFrameAligned,
    ByteLengthMismatch { expected: usize, actual: usize },
    FrameCountOverflow,
    ByteLengthOverflow,
}

pub(crate) fn validate_render_format(
    format: PcmRenderFormat,
) -> Result<PcmRenderFormat, PcmAdapterError> {
    if format.sample_rate_hz == 0 {
        return Err(PcmAdapterError::ZeroSampleRate);
    }

    if format.channels == 0 {
        return Err(PcmAdapterError::ZeroChannels);
    }

    if format.block_align == 0 {
        return Err(PcmAdapterError::InvalidBlockAlign);
    }

    if !format.block_align.is_multiple_of(format.channels) {
        return Err(PcmAdapterError::InvalidBlockAlign);
    }

    match format.sample_format {
        PcmSampleFormat::Float32Interleaved => validate_float32_interleaved(format),
        PcmSampleFormat::Signed16Interleaved
        | PcmSampleFormat::Signed24Interleaved
        | PcmSampleFormat::Signed32Interleaved => Err(PcmAdapterError::UnsupportedSampleFormat),
    }
}

pub(crate) fn frames_for_f32_samples(
    format: PcmRenderFormat,
    samples: &[f32],
) -> Result<u32, PcmAdapterError> {
    let format = validate_render_format(format)?;
    let channels = usize::from(format.channels);

    if !samples.len().is_multiple_of(channels) {
        return Err(PcmAdapterError::SampleCountNotFrameAligned);
    }

    u32::try_from(samples.len() / channels).map_err(|_| PcmAdapterError::FrameCountOverflow)
}

pub(crate) fn f32_interleaved_samples_to_bytes(
    format: PcmRenderFormat,
    samples: &[f32],
) -> Result<Vec<u8>, PcmAdapterError> {
    let format = validate_render_format(format)?;
    let frames = frames_for_f32_samples(format, samples)?;
    let expected_len = expected_byte_len(frames, format.block_align)?;
    let mut bytes = Vec::with_capacity(expected_len);

    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }

    if bytes.len() != expected_len {
        return Err(PcmAdapterError::ByteLengthMismatch {
            expected: expected_len,
            actual: bytes.len(),
        });
    }

    Ok(bytes)
}

pub(crate) fn validate_render_bytes<'a>(
    format: PcmRenderFormat,
    frames: u32,
    bytes: &'a [u8],
) -> Result<PcmFrameBytes<'a>, PcmAdapterError> {
    let format = validate_render_format(format)?;
    let expected_len = expected_byte_len(frames, format.block_align)?;

    if bytes.len() != expected_len {
        return Err(PcmAdapterError::ByteLengthMismatch {
            expected: expected_len,
            actual: bytes.len(),
        });
    }

    Ok(PcmFrameBytes { frames, bytes })
}

fn validate_float32_interleaved(
    format: PcmRenderFormat,
) -> Result<PcmRenderFormat, PcmAdapterError> {
    if format.bits_per_sample != FLOAT32_BITS_PER_SAMPLE {
        return Err(PcmAdapterError::InvalidBitsPerSample);
    }

    if usize::from(format.block_align) != expected_float32_block_align(format.channels) {
        return Err(PcmAdapterError::InvalidBlockAlign);
    }

    // Channel masks and WAVEFORMATEXTENSIBLE metadata are deliberately outside
    // this pure PCM byte contract; the future device layer owns that boundary.
    Ok(format)
}

fn expected_float32_block_align(channels: u16) -> usize {
    usize::from(channels) * usize::from(FLOAT32_BYTES_PER_SAMPLE)
}

fn expected_byte_len(frames: u32, block_align: u16) -> Result<usize, PcmAdapterError> {
    let frames = usize::try_from(frames).map_err(|_| PcmAdapterError::ByteLengthOverflow)?;

    frames
        .checked_mul(usize::from(block_align))
        .ok_or(PcmAdapterError::ByteLengthOverflow)
}
