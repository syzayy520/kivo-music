use crate::playback::decoder::AudioSampleFormat;

use super::fixtures::{float_stream, opened, output_frame, stream};

#[test]
fn query_returns_current_report_without_detaching() -> Result<(), String> {
    let (policy, mut pipeline) = opened(2)?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));

    let report = policy
        .current_tap_report(&pipeline)
        .ok_or_else(|| "current report missing".to_string())?;

    assert_eq!(report.input_count, 1);
    assert!(pipeline.audio_route_pipeline_tap_report().is_some());
    Ok(())
}

#[test]
fn query_returns_only_latest_detached_report() -> Result<(), String> {
    let (mut policy, mut pipeline) = opened(2)?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));
    let closed = policy
        .close_detach_on_stop(&mut pipeline)
        .ok_or_else(|| "closed report missing".to_string())?;

    assert_eq!(policy.last_detached_report(), Some(closed));
    assert!(policy.current_tap_report(&pipeline).is_none());
    Ok(())
}

#[test]
fn compatibility_query_uses_all_stream_fields() -> Result<(), String> {
    let (policy, _pipeline) = opened(2)?;
    let different_format = stream(48_000, 2, AudioSampleFormat::Signed16);

    assert!(policy.current_stream_is_compatible(&float_stream()));
    assert!(!policy.current_stream_is_compatible(&different_format));
    Ok(())
}
