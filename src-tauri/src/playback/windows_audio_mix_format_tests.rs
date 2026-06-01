use super::windows_audio_mix_format::WindowsAudioMixFormat;

#[test]
fn mix_format_preserves_pcm_stereo_shape() {
    let format = WindowsAudioMixFormat {
        format_tag: 1,
        channels: 2,
        sample_rate: 44_100,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 176_400,
    };

    assert_eq!(format.format_tag, 1);
    assert_eq!(format.channels, 2);
    assert_eq!(format.sample_rate, 44_100);
    assert_eq!(format.bits_per_sample, 16);
    assert_eq!(format.block_align, 4);
    assert_eq!(format.avg_bytes_per_sec, 176_400);
}

#[test]
fn mix_format_preserves_float_surround_shape() {
    let format = WindowsAudioMixFormat {
        format_tag: 3,
        channels: 6,
        sample_rate: 48_000,
        bits_per_sample: 32,
        block_align: 24,
        avg_bytes_per_sec: 1_152_000,
    };

    assert_eq!(format.format_tag, 3);
    assert_eq!(format.channels, 6);
    assert_eq!(format.sample_rate, 48_000);
    assert_eq!(format.bits_per_sample, 32);
    assert_eq!(format.block_align, 24);
    assert_eq!(format.avg_bytes_per_sec, 1_152_000);
}
