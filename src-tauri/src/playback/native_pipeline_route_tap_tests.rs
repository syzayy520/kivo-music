use super::audio_route_pipeline_tap::{AudioRoutePipelineTap, AudioRoutePipelineTapConfig};
use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::native_pipeline::NativePipeline;
use super::output::AudioOutputFrame;

fn stream_info() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn output_frame(samples: Vec<f32>) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream_info(),
        position_ms: 42,
        samples,
    }
}

fn tap(capacity_frames: u32) -> Result<AudioRoutePipelineTap, String> {
    AudioRoutePipelineTapConfig::new(stream_info(), capacity_frames)
        .and_then(AudioRoutePipelineTap::new)
        .map_err(|error| format!("{error:?}"))
}

#[test]
fn native_pipeline_tap_hook_noops_without_attached_tap() {
    let mut pipeline = NativePipeline::new();
    let frame = output_frame(vec![0.0, 0.1]);

    pipeline.tap_audio_route_output_frame_non_fatal(&frame);

    assert!(pipeline.audio_route_pipeline_tap_report().is_none());
}

#[test]
fn native_pipeline_attached_tap_records_side_report() -> Result<(), String> {
    let mut pipeline = NativePipeline::new();
    let frame = output_frame(vec![0.0, 0.1]);
    let previous = pipeline.attach_audio_route_pipeline_tap(tap(2)?);

    pipeline.tap_audio_route_output_frame_non_fatal(&frame);
    let report = pipeline
        .audio_route_pipeline_tap_report()
        .ok_or_else(|| "tap report missing".to_string())?;

    assert!(previous.is_none());
    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.input_count, 1);
    Ok(())
}

#[test]
fn native_pipeline_hook_does_not_block_enqueue() -> Result<(), String> {
    let mut pipeline = NativePipeline::new();
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    pipeline.attach_audio_route_pipeline_tap(tap(1)?);

    pipeline.tap_audio_route_output_frame_non_fatal(&frame);
    pipeline.enqueue_decoded_frame(frame);
    pipeline
        .schedule_output_submit_step()
        .map_err(|error| format!("{error:?}"))?;
    let report = pipeline
        .audio_route_pipeline_tap_report()
        .ok_or_else(|| "tap report missing".to_string())?;

    assert_eq!(report.backpressure_count, 1);
    assert_eq!(report.partial_write_count, 1);
    Ok(())
}

#[test]
fn decoder_hook_is_before_clone_and_enqueue() -> Result<(), String> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/playback/native_pipeline_decoder.rs");
    let text = std::fs::read_to_string(path).map_err(|error| format!("{error}"))?;
    let hook = text
        .find("tap_audio_route_output_frame_non_fatal(&output_frame)")
        .ok_or_else(|| "tap hook missing".to_string())?;
    let last_frame = text
        .find("last_decoded_frame = Some(output_frame.clone())")
        .ok_or_else(|| "last decoded frame assignment missing".to_string())?;
    let enqueue = text
        .find("enqueue_decoded_frame(output_frame)")
        .ok_or_else(|| "enqueue missing".to_string())?;

    assert!(hook < last_frame);
    assert!(hook < enqueue);
    Ok(())
}
