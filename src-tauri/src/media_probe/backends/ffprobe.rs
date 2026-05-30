use std::process::Command;

use serde_json::Value;

use super::super::service::{MediaProbeError, MediaProbeResultValue, ProbeBackend};
use super::super::types::{AudioProbeResult, MediaProbeResult, VideoProbeResult};

#[derive(Clone, Debug, Default)]
pub struct FfprobeBackend;

impl ProbeBackend for FfprobeBackend {
    fn name(&self) -> &'static str {
        "ffprobe"
    }

    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
            ])
            .arg(path)
            .output()
            .map_err(|error| MediaProbeError::BackendUnavailable(error.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(MediaProbeError::ProbeFailed(stderr));
        }

        let root: Value = serde_json::from_slice(&output.stdout)
            .map_err(|error| MediaProbeError::ProbeFailed(error.to_string()))?;

        Ok(parse_probe_result(path, &root))
    }
}

fn parse_probe_result(path: &str, root: &Value) -> MediaProbeResult {
    let streams = root
        .get("streams")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let format = root.get("format");

    MediaProbeResult {
        path: path.to_string(),
        audio: parse_audio_probe(streams, format),
        video: parse_video_probe(streams, format),
    }
}

fn parse_audio_probe(streams: &[Value], format: Option<&Value>) -> Option<AudioProbeResult> {
    let stream = first_stream(streams, "audio", false)?;

    Some(AudioProbeResult {
        duration_ms: duration_ms(stream).or_else(|| format.and_then(duration_ms)),
        codec: string_field(stream, "codec_name"),
        container: format.and_then(|value| string_field(value, "format_name")),
        sample_rate_hz: string_field(stream, "sample_rate").and_then(|value| value.parse().ok()),
        bit_depth: bit_depth(stream),
        channels: number_field(stream, "channels").map(|value| value as u16),
        bitrate_kbps: bit_rate_kbps(stream).or_else(|| format.and_then(bit_rate_kbps)),
        is_lossless: string_field(stream, "codec_name").map(|codec| is_lossless_codec(&codec)),
        has_embedded_cover: has_attached_picture(streams),
    })
}

fn parse_video_probe(streams: &[Value], format: Option<&Value>) -> Option<VideoProbeResult> {
    let stream = first_stream(streams, "video", true)?;

    Some(VideoProbeResult {
        duration_ms: duration_ms(stream).or_else(|| format.and_then(duration_ms)),
        codec: string_field(stream, "codec_name"),
        container: format.and_then(|value| string_field(value, "format_name")),
        width: number_field(stream, "width"),
        height: number_field(stream, "height"),
        frame_rate: string_field(stream, "avg_frame_rate").or_else(|| string_field(stream, "r_frame_rate")),
        hdr_format: hdr_format(stream),
        dolby_vision_profile: dolby_vision_profile(stream),
        audio_layout: first_stream(streams, "audio", false).and_then(audio_layout),
    })
}

fn first_stream<'a>(streams: &'a [Value], kind: &str, skip_attached_picture: bool) -> Option<&'a Value> {
    streams.iter().find(|stream| {
        string_field(stream, "codec_type").as_deref() == Some(kind)
            && (!skip_attached_picture || !is_attached_picture(stream))
    })
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

fn number_field(value: &Value, key: &str) -> Option<u32> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
}

fn duration_ms(value: &Value) -> Option<u64> {
    string_field(value, "duration")
        .and_then(|duration| duration.parse::<f64>().ok())
        .map(|seconds| (seconds * 1000.0).round() as u64)
}

fn bit_rate_kbps(value: &Value) -> Option<u32> {
    string_field(value, "bit_rate")
        .and_then(|bit_rate| bit_rate.parse::<u32>().ok())
        .map(|bit_rate| bit_rate / 1000)
}

fn bit_depth(stream: &Value) -> Option<u16> {
    number_field(stream, "bits_per_sample")
        .or_else(|| number_field(stream, "bits_per_raw_sample"))
        .map(|value| value as u16)
}

fn audio_layout(stream: &Value) -> Option<String> {
    string_field(stream, "channel_layout").or_else(|| {
        number_field(stream, "channels").map(|channels| format!("{channels} channels"))
    })
}

fn hdr_format(stream: &Value) -> Option<String> {
    let transfer = string_field(stream, "color_transfer")?;

    match transfer.as_str() {
        "smpte2084" => Some("HDR10/PQ".to_string()),
        "arib-std-b67" => Some("HLG".to_string()),
        _ => None,
    }
}

fn dolby_vision_profile(stream: &Value) -> Option<String> {
    let side_data = stream.get("side_data_list")?.as_array()?;

    side_data.iter().find_map(|item| {
        let side_data_type = string_field(item, "side_data_type")?;
        if !side_data_type.to_ascii_lowercase().contains("dovi") {
            return None;
        }

        string_field(item, "dv_profile").or_else(|| Some("detected".to_string()))
    })
}

fn has_attached_picture(streams: &[Value]) -> bool {
    streams.iter().any(is_attached_picture)
}

fn is_attached_picture(stream: &Value) -> bool {
    stream
        .get("disposition")
        .and_then(|disposition| disposition.get("attached_pic"))
        .and_then(Value::as_i64)
        == Some(1)
}

fn is_lossless_codec(codec: &str) -> bool {
    matches!(
        codec.to_ascii_lowercase().as_str(),
        "alac" | "ape" | "flac" | "pcm_s16le" | "pcm_s24le" | "pcm_s32le" | "wavpack"
    )
}
