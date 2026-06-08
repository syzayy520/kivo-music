use crate::playback::native_pipeline::NativePipeline;

#[test]
fn progress_snapshot_reflects_default_pipeline_state() {
    let pipeline = NativePipeline::new();
    let progress = pipeline.progress_snapshot();

    assert_eq!(progress.position_ms, 0);
    assert!(!progress.is_started);
    assert!(!progress.is_paused);
    assert_eq!(progress.buffered_frames, 0);
    assert_eq!(progress.output_pending_frames, 0);
}

#[test]
fn progress_snapshot_reflects_started_clock_state() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");

    let progress = pipeline.progress_snapshot();
    assert!(progress.is_started);
    assert!(!progress.is_paused);
    assert_eq!(progress.position_ms, 0);
}

#[test]
fn progress_snapshot_reflects_paused_clock_state() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");
    pipeline.pause_output().expect("pause output");

    let progress = pipeline.progress_snapshot();
    assert!(progress.is_started);
    assert!(progress.is_paused);
}

#[test]
fn progress_snapshot_reflects_resumed_clock_state() {
    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");
    pipeline.pause_output().expect("pause output");
    pipeline.resume_output().expect("resume output");

    let progress = pipeline.progress_snapshot();
    assert!(progress.is_started);
    assert!(!progress.is_paused);
}

#[test]
fn progress_snapshot_reflects_drained_position() {
    use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
    use crate::playback::output::AudioOutputFrame;

    let mut pipeline = NativePipeline::new();
    pipeline.start().expect("start null sink");

    let frame = AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 500,
        samples: vec![0.0, 0.0],
    };
    pipeline.enqueue_decoded_frame(frame);
    pipeline
        .drain_next_frame_to_output()
        .expect("drain should succeed");

    let progress = pipeline.progress_snapshot();
    assert_eq!(progress.position_ms, 500);
    assert!(progress.is_started);
    assert_eq!(progress.buffered_frames, 0);
    assert_eq!(progress.output_pending_frames, 1);
}
