use super::fixtures::{chunk, closed_chunk, owner};

#[test]
fn report_aggregates_multiple_successful_feeds() -> Result<(), String> {
    let first = [0.0, 0.1];
    let second = [0.2, 0.3, 0.4, 0.5];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    owner
        .feed_pcm_source_chunk(chunk(&first))
        .map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(chunk(&second))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.total_requested_frames, 3);
    assert_eq!(report.total_accepted_frames, 3);
    assert_eq!(report.total_rejected_frames, 0);
    assert_eq!(report.last_requested_frames, 2);
    assert_eq!(report.pending_frames, 3);
    Ok(())
}

#[test]
fn source_closed_empty_samples_updates_last_fields() -> Result<(), String> {
    let samples = [];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(closed_chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.last_source_closed);
    assert_eq!(report.last_requested_frames, 0);
    assert_eq!(report.total_accepted_frames, 0);
    Ok(())
}

#[test]
fn source_closed_non_empty_samples_write_final_frames() -> Result<(), String> {
    let samples = [0.0, 0.1];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(closed_chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.last_source_closed);
    assert_eq!(report.last_accepted_frames, 1);
    assert_eq!(owner.pending_frames(), 1);
    Ok(())
}
