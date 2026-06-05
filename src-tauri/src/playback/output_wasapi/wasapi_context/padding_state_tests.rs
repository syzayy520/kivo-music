use super::{
    calculate_available_frames, WasapiContext, WasapiPaddingStateError, WasapiPaddingStateSnapshot,
};

#[test]
fn padding_state_closed_context_returns_not_open() {
    let context = WasapiContext::new();

    assert_eq!(
        context.buffer_frame_capacity(),
        Err(WasapiPaddingStateError::NotOpen)
    );
    assert_eq!(
        context.current_padding_frames(),
        Err(WasapiPaddingStateError::NotOpen)
    );
    assert_eq!(
        context.available_frames(),
        Err(WasapiPaddingStateError::NotOpen)
    );
    assert_eq!(
        context.padding_state_snapshot(),
        Err(WasapiPaddingStateError::NotOpen)
    );
}

#[test]
fn calculate_available_frames_returns_capacity_minus_padding() {
    assert_eq!(calculate_available_frames(256, 64), Ok(192));
}

#[test]
fn calculate_available_frames_allows_full_padding() {
    assert_eq!(calculate_available_frames(128, 128), Ok(0));
}

#[test]
fn calculate_available_frames_rejects_padding_exceeding_capacity() {
    assert_eq!(
        calculate_available_frames(32, 33),
        Err(WasapiPaddingStateError::PaddingExceedsCapacity {
            padding: 33,
            capacity: 32,
        })
    );
}

#[test]
fn padding_state_snapshot_keeps_capacity_padding_and_available() {
    let snapshot = WasapiPaddingStateSnapshot {
        buffer_frame_capacity: 480,
        current_padding_frames: 120,
        available_frames: calculate_available_frames(480, 120).expect("valid padding"),
    };

    assert_eq!(snapshot.buffer_frame_capacity, 480);
    assert_eq!(snapshot.current_padding_frames, 120);
    assert_eq!(snapshot.available_frames, 360);
}

#[cfg(target_os = "windows")]
#[test]
#[ignore = "requires a real Windows default render endpoint"]
fn windows_padding_state_smoke_ignored() {
    let mut context = WasapiContext::new();

    context.open().expect("open real WASAPI context");
    let snapshot = context
        .padding_state_snapshot()
        .expect("read padding state snapshot");
    assert!(snapshot.buffer_frame_capacity >= snapshot.current_padding_frames);
    assert_eq!(
        snapshot.available_frames,
        snapshot.buffer_frame_capacity - snapshot.current_padding_frames
    );
    context.close();
}
