use std::fs;

use crate::playback::native_pipeline::NativePipeline;
use crate::playback::playback_worker_command::PlaybackWorkerCommand;
use crate::playback::types::{PlaybackTrack, TrackId};

use super::helpers::write_test_wav;

#[test]
fn worker_load_decodes_buffer_drains_to_null_sink() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let track = PlaybackTrack {
        id: TrackId("worker-buffer-drain-1".to_string()),
        title: "Worker Buffer Drain".to_string(),
        artist: "Worker Artist".to_string(),
        source_path: path.clone(),
    };

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Load { track })
        .expect("worker load wav");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.last_error.is_none());
    assert_eq!(pipeline.buffered_frame_count(), 0);
    assert!(pipeline.clock.is_started());
    assert!(!pipeline.clock.is_paused());

    pipeline.shutdown().expect("shutdown pipeline");
    fs::remove_file(path).expect("remove wav test file");
}
