use super::pcm_adapter::{
    f32_interleaved_samples_to_bytes, frames_for_f32_samples, validate_render_bytes,
    validate_render_format, PcmAdapterError, PcmRenderFormat, PcmSampleFormat,
};

fn format(channels: u16) -> PcmRenderFormat {
    PcmRenderFormat {
        sample_rate_hz: 44_100,
        channels,
        bits_per_sample: 32,
        block_align: channels * 4,
        sample_format: PcmSampleFormat::Float32Interleaved,
    }
}

#[test]
fn pcm_adapter_accepts_float32_interleaved_stereo_44100() {
    let samples = [0.25, -0.25, 0.5, -0.5];

    assert_eq!(frames_for_f32_samples(format(2), &samples), Ok(2));

    let bytes = f32_interleaved_samples_to_bytes(format(2), &samples).expect("bytes");
    assert_eq!(bytes.len(), 16);
}

#[test]
fn pcm_adapter_accepts_float32_interleaved_mono() {
    let samples = [0.0, 0.5, 1.0];

    assert_eq!(frames_for_f32_samples(format(1), &samples), Ok(3));
}

#[test]
fn pcm_adapter_accepts_float32_interleaved_multichannel() {
    let samples = [0.0_f32; 12];

    assert_eq!(frames_for_f32_samples(format(6), &samples), Ok(2));
}

#[test]
fn pcm_adapter_converts_f32_samples_to_little_endian_bytes() {
    let samples = [1.0_f32, -0.5_f32];
    let expected: Vec<u8> = samples
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect();

    assert_eq!(
        f32_interleaved_samples_to_bytes(format(2), &samples),
        Ok(expected)
    );
}

#[test]
fn pcm_adapter_rejects_zero_channels() {
    let mut pcm_format = format(1);
    pcm_format.channels = 0;

    assert_eq!(
        validate_render_format(pcm_format),
        Err(PcmAdapterError::ZeroChannels)
    );
}

#[test]
fn pcm_adapter_rejects_zero_sample_rate() {
    let mut pcm_format = format(2);
    pcm_format.sample_rate_hz = 0;

    assert_eq!(
        validate_render_format(pcm_format),
        Err(PcmAdapterError::ZeroSampleRate)
    );
}

#[test]
fn pcm_adapter_rejects_non_float32_sample_format() {
    let pcm_format = PcmRenderFormat {
        sample_rate_hz: 48_000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        sample_format: PcmSampleFormat::Signed16Interleaved,
    };

    assert_eq!(
        validate_render_format(pcm_format),
        Err(PcmAdapterError::UnsupportedSampleFormat)
    );
}

#[test]
fn pcm_adapter_rejects_reserved_integer_formats() {
    for sample_format in [
        PcmSampleFormat::Signed16Interleaved,
        PcmSampleFormat::Signed24Interleaved,
        PcmSampleFormat::Signed32Interleaved,
    ] {
        let pcm_format = PcmRenderFormat {
            sample_rate_hz: 48_000,
            channels: 2,
            bits_per_sample: 16,
            block_align: 4,
            sample_format,
        };

        assert_eq!(
            validate_render_format(pcm_format),
            Err(PcmAdapterError::UnsupportedSampleFormat)
        );
    }
}

#[test]
fn pcm_adapter_rejects_float32_bits_per_sample_not_32() {
    let mut pcm_format = format(2);
    pcm_format.bits_per_sample = 24;

    assert_eq!(
        validate_render_format(pcm_format),
        Err(PcmAdapterError::InvalidBitsPerSample)
    );
}

#[test]
fn pcm_adapter_rejects_invalid_block_align_for_channels() {
    let mut pcm_format = format(2);
    pcm_format.block_align = 12;

    assert_eq!(
        validate_render_format(pcm_format),
        Err(PcmAdapterError::InvalidBlockAlign)
    );
}

#[test]
fn pcm_adapter_rejects_sample_count_not_divisible_by_channels() {
    let samples = [0.0_f32, 0.5, 1.0];

    assert_eq!(
        frames_for_f32_samples(format(2), &samples),
        Err(PcmAdapterError::SampleCountNotFrameAligned)
    );
}

#[test]
fn pcm_adapter_rejects_bytes_length_mismatch() {
    assert_eq!(
        validate_render_bytes(format(2), 2, &[0_u8; 15]),
        Err(PcmAdapterError::ByteLengthMismatch {
            expected: 16,
            actual: 15
        })
    );
}

#[test]
fn pcm_adapter_validate_render_bytes_accepts_exact_frames_times_block_align() {
    let bytes = [0_u8; 16];
    let frame_bytes = validate_render_bytes(format(2), 2, &bytes).expect("frame bytes");

    assert_eq!(frame_bytes.frames, 2);
    assert_eq!(frame_bytes.bytes, &bytes);
}

#[test]
fn pcm_adapter_validate_render_bytes_rejects_overflow_or_mismatch() {
    let result = validate_render_bytes(format(2), u32::MAX, &[]);

    assert!(matches!(
        result,
        Err(PcmAdapterError::ByteLengthOverflow) | Err(PcmAdapterError::ByteLengthMismatch { .. })
    ));
}

#[test]
fn pcm_adapter_empty_input_behavior_is_documented() {
    let bytes = f32_interleaved_samples_to_bytes(format(2), &[]).expect("empty bytes");
    let frame_bytes = validate_render_bytes(format(2), 0, &bytes).expect("empty frame bytes");

    assert_eq!(frames_for_f32_samples(format(2), &[]), Ok(0));
    assert!(bytes.is_empty());
    assert_eq!(frame_bytes.frames, 0);
    assert!(frame_bytes.bytes.is_empty());
}
