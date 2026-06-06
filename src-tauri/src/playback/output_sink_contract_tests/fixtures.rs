use super::super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::super::native_null_output::KivoNullOutputSink;
use super::super::output::{AudioOutputFrame, OutputSettings};

pub(super) fn null_sink() -> KivoNullOutputSink {
    KivoNullOutputSink::new()
}

pub(super) fn output_settings(selected_device_id: Option<&str>) -> OutputSettings {
    OutputSettings {
        selected_device_id: selected_device_id.map(str::to_string),
        ..OutputSettings::default()
    }
}

pub(super) fn output_frame(position_ms: u64) -> AudioOutputFrame {
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
