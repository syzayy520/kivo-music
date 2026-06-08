use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::native_pipeline_route_tap_diagnostic_policy::{
    NativePipelineRouteTapDiagnosticPolicy, NativeTapDiagnosticConfig,
};
use crate::playback::output::AudioOutputFrame;

pub(super) fn stream(
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

pub(super) fn float_stream() -> AudioStreamInfo {
    stream(48_000, 2, AudioSampleFormat::Float32)
}

pub(super) fn policy(
    capacity_frames: u32,
) -> Result<NativePipelineRouteTapDiagnosticPolicy, String> {
    NativeTapDiagnosticConfig::new(capacity_frames)
        .map(NativePipelineRouteTapDiagnosticPolicy::new)
        .map_err(|error| format!("{error:?}"))
}

pub(super) fn opened(
    capacity_frames: u32,
) -> Result<(NativePipelineRouteTapDiagnosticPolicy, NativePipeline), String> {
    let mut policy = policy(capacity_frames)?;
    let mut pipeline = NativePipeline::new();
    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;
    Ok((policy, pipeline))
}

pub(super) fn output_frame(samples: Vec<f32>) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: float_stream(),
        position_ms: 42,
        samples,
    }
}
