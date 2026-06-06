use super::fixtures::{chunk, decoded_frame, output_frame, owner};

#[test]
fn feed_pcm_source_chunk_writes_frames() -> Result<(), String> {
    let samples = [0.0, 0.25, 0.5, 0.75];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.last_requested_frames, 2);
    assert_eq!(report.last_accepted_frames, 2);
    assert_eq!(owner.pending_frames(), 2);
    Ok(())
}

#[test]
fn feed_decoded_frame_adapter_works() -> Result<(), String> {
    let frame = decoded_frame(vec![0.0, 0.1]);
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_decoded_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.last_accepted_frames, 1);
    assert_eq!(report.total_accepted_frames, 1);
    Ok(())
}

#[test]
fn feed_output_frame_adapter_works() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_output_frame(&frame)
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.last_accepted_frames, 2);
    assert_eq!(owner.pending_frames(), 2);
    Ok(())
}

#[test]
fn empty_samples_update_noop_report() -> Result<(), String> {
    let samples = [];
    let mut owner = owner(4).map_err(|error| format!("{error:?}"))?;
    let report = owner
        .feed_pcm_source_chunk(chunk(&samples))
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.last_requested_frames, 0);
    assert_eq!(report.last_accepted_frames, 0);
    assert_eq!(report.pending_frames, 0);
    Ok(())
}
