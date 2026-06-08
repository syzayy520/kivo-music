use super::fixtures::{chunk, coordinator, decoded_frame, output_frame};

#[test]
fn feed_pcm_source_chunk_updates_route_report() -> Result<(), String> {
    let samples = [0.0, 0.25, 0.5, 0.75];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_requested_frames, 2);
    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    assert_eq!(report.feed_count, 1);
    Ok(())
}

#[test]
fn feed_decoded_frame_adapter_delegates_to_route_owner() -> Result<(), String> {
    let frame = decoded_frame(vec![0.0, 0.1]);
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_decoded_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.feed_count, 1);
    Ok(())
}

#[test]
fn feed_output_frame_adapter_delegates_to_route_owner() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_output_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    Ok(())
}
