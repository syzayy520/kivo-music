use crate::playback::audio_route_integration::frame_input::AudioRouteFrameInput;

use super::fixtures::{chunk, decoded_frame, integration, output_frame};

#[test]
fn feed_pcm_source_chunk_updates_integration_report() -> Result<(), String> {
    let samples = [0.0, 0.25, 0.5, 0.75];
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    let report = integration
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_requested_frames, 2);
    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    assert_eq!(report.input_count, 1);
    Ok(())
}

#[test]
fn feed_decoded_frame_adapter_delegates_to_coordinator() -> Result<(), String> {
    let frame = decoded_frame(vec![0.0, 0.1]);
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    let report = integration
        .feed_decoded_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.input_count, 1);
    Ok(())
}

#[test]
fn feed_output_frame_adapter_delegates_to_coordinator() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    let report = integration
        .feed_output_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    Ok(())
}

#[test]
fn feed_frame_input_accepts_pcm_source_chunk() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut integration = integration(4).map_err(|error| format!("{error:?}"))?;
    let report = integration
        .feed_frame_input(AudioRouteFrameInput::PcmSourceChunk(chunk(&samples)))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.input_count, 1);
    Ok(())
}
