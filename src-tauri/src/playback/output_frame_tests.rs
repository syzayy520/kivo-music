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
