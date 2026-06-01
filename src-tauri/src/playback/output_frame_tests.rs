use super::decoder::{AudioSampleFormat, AudioStreamInfo, DecodedAudioFrame};
use super::output_frame::OutputAudioFrame;

#[test]
fn output_frame_keeps_stream_position_and_samples() {
    let decoded = DecodedAudioFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 12_345,
        samples: vec![0.1, -0.1, 0.5, -0.5],
    };

    let output = OutputAudioFrame::from_decoded_frame(decoded);

    assert_eq!(output.stream.sample_rate_hz, 48_000);
    assert_eq!(output.stream.channels, 2);
    assert_eq!(output.position_ms, 12_345);
    assert_eq!(output.samples, vec![0.1, -0.1, 0.5, -0.5]);
}

#[test]
fn output_frame_preserves_sample_format() {
    let decoded = DecodedAudioFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 96_000,
            channels: 6,
            sample_format: AudioSampleFormat::Signed24,
        },
        position_ms: 250,
        samples: vec![0.25, -0.25],
    };

    let output = OutputAudioFrame::from_decoded_frame(decoded);

    assert!(matches!(
        output.stream.sample_format,
        AudioSampleFormat::Signed24
    ));
}

#[test]
fn output_frame_allows_empty_samples() {
    let decoded = DecodedAudioFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 44_100,
            channels: 1,
            sample_format: AudioSampleFormat::Signed16,
        },
        position_ms: 0,
        samples: vec![],
    };

    let output = OutputAudioFrame::from_decoded_frame(decoded);

    assert_eq!(output.stream.sample_rate_hz, 44_100);
    assert_eq!(output.stream.channels, 1);
    assert_eq!(output.position_ms, 0);
    assert!(output.samples.is_empty());
}
