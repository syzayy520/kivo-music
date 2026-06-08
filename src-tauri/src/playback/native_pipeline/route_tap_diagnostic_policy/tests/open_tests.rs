use crate::playback::decoder::AudioSampleFormat;
use crate::playback::native_pipeline::NativePipeline;

use super::fixtures::{float_stream, output_frame, policy, stream};

#[test]
fn open_without_existing_tap_attaches_fresh_diagnostic_tap() -> Result<(), String> {
    let mut policy = policy(4)?;
    let mut pipeline = NativePipeline::new();

    let report = policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.initialized);
    assert!(!report.closed);
    assert_eq!(report.capacity_frames, 4);
    assert!(pipeline.audio_route_pipeline_tap_report().is_some());
    assert!(policy.last_detached_report().is_none());
    Ok(())
}

#[test]
fn opening_same_stream_still_closes_and_recreates_tap() -> Result<(), String> {
    let mut policy = policy(2)?;
    let mut pipeline = NativePipeline::new();
    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));

    let fresh = policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;
    let detached = policy
        .last_detached_report()
        .ok_or_else(|| "last detached report missing".to_string())?;

    assert!(detached.closed);
    assert_eq!(detached.input_count, 1);
    assert_eq!(fresh.input_count, 0);
    assert_eq!(fresh.pending_frames, 0);
    Ok(())
}

#[test]
fn opening_different_stream_closes_and_recreates_tap() -> Result<(), String> {
    let mut policy = policy(2)?;
    let mut pipeline = NativePipeline::new();
    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;

    let replacement = stream(44_100, 1, AudioSampleFormat::Float32);
    let fresh = policy
        .open_new_track(&mut pipeline, replacement)
        .map_err(|error| format!("{error:?}"))?;

    assert!(policy
        .last_detached_report()
        .is_some_and(|report| report.closed));
    assert_eq!(fresh.input_count, 0);
    assert_eq!(
        policy.current_stream().map(|stream| stream.channels),
        Some(1)
    );
    Ok(())
}

#[test]
fn each_recreate_overwrites_the_single_last_detached_report() -> Result<(), String> {
    let mut policy = policy(4)?;
    let mut pipeline = NativePipeline::new();
    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));
    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.2, 0.3]));

    policy
        .open_new_track(&mut pipeline, float_stream())
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(
        policy
            .last_detached_report()
            .map(|report| report.input_count),
        Some(2)
    );
    Ok(())
}
