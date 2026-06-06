use super::fixtures::{chunk, closed_chunk, coordinator};

#[test]
fn report_aggregates_multiple_successful_feeds() -> Result<(), String> {
    let first = [0.0, 0.1];
    let second = [0.2, 0.3];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    coordinator
        .feed_pcm_source_chunk(chunk(&first))
        .map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_pcm_source_chunk(chunk(&second))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_requested_frames, 2);
    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.total_rejected_frames, 0);
    assert_eq!(report.feed_count, 2);
    Ok(())
}

#[test]
fn partial_write_updates_compact_counts() -> Result<(), String> {
    let samples = [0.0, 0.1, 0.2, 0.3];
    let mut coordinator = coordinator(1).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_requested_frames, 2);
    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.total_rejected_frames, 1);
    assert_eq!(report.partial_write_count, 1);
    assert_eq!(report.backpressure_count, 1);
    Ok(())
}

#[test]
fn source_closed_empty_samples_is_reflected() -> Result<(), String> {
    let samples = [];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_pcm_source_chunk(closed_chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.source_closed_seen);
    assert_eq!(report.feed_count, 1);
    Ok(())
}

#[test]
fn source_closed_non_empty_samples_is_reflected() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut coordinator = coordinator(4).map_err(|error| format!("{error:?}"))?;
    let report = coordinator
        .feed_pcm_source_chunk(closed_chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.source_closed_seen);
    assert_eq!(report.total_accepted_frames, 1);
    Ok(())
}
