use super::windows_audio_client_init::WindowsAudioClientInitSnapshot;
use super::windows_audio_mix_format::WindowsAudioMixFormat;

#[test]
fn client_init_snapshot_can_model_uninitialized_state() {
    let snapshot = WindowsAudioClientInitSnapshot {
        initialized: false,
        buffer_frame_count: None,
        mix_format: None,
        note: Some("not initialized".to_string()),
    };

    assert!(!snapshot.initialized);
    assert!(snapshot.buffer_frame_count.is_none());
    assert!(snapshot.mix_format.is_none());
    assert_eq!(snapshot.note.as_deref(), Some("not initialized"));
}

#[test]
fn client_init_snapshot_preserves_buffer_and_mix_format() {
    let snapshot = WindowsAudioClientInitSnapshot {
        initialized: true,
        buffer_frame_count: Some(4800),
        mix_format: Some(WindowsAudioMixFormat {
            format_tag: 1,
            channels: 2,
            sample_rate: 48_000,
            bits_per_sample: 32,
            block_align: 8,
            avg_bytes_per_sec: 384_000,
        }),
        note: None,
    };

    let mix_format = snapshot.mix_format.expect("mix format should be present");

    assert!(snapshot.initialized);
    assert_eq!(snapshot.buffer_frame_count, Some(4800));
    assert_eq!(mix_format.channels, 2);
    assert_eq!(mix_format.sample_rate, 48_000);
    assert_eq!(mix_format.bits_per_sample, 32);
    assert!(snapshot.note.is_none());
}
