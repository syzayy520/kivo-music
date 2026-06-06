use std::mem;

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;

use super::super::ProductionOutputRouteFrameInput;

fn frame(position_ms: u64) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms,
        samples: vec![0.0, 0.25, -0.25, 0.0],
    }
}

#[test]
fn production_output_route_input_accepts_audio_output_frame_metadata() {
    let frame = frame(1_250);
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    assert_eq!(input.position_ms(), 1_250);
    assert_eq!(input.stream().sample_rate_hz, 48_000);
    assert_eq!(input.stream().channels, 2);
    assert_eq!(
        mem::discriminant(&input.stream().sample_format),
        mem::discriminant(&AudioSampleFormat::Float32)
    );
    assert_eq!(input.sample_count(), 4);
}

#[test]
fn production_output_route_input_borrows_without_mutating_frame() {
    let frame = frame(640);
    let original_samples = frame.samples.clone();
    let original_position = frame.position_ms;

    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    assert_eq!(input.position_ms(), original_position);
    assert_eq!(input.sample_count(), original_samples.len());
    assert_eq!(frame.position_ms, original_position);
    assert_eq!(frame.samples, original_samples);
}

#[test]
fn production_output_route_input_keeps_samples_inside_frame_contract() {
    let frame = frame(320);
    let input = ProductionOutputRouteFrameInput::from_frame(&frame);

    assert_eq!(input.sample_count(), frame.samples.len());
    assert_eq!(input.stream().sample_rate_hz, frame.stream.sample_rate_hz);
    assert_eq!(input.stream().channels, frame.stream.channels);
}
