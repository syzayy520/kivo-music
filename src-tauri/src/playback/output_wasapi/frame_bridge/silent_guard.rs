use crate::playback::output::AudioOutputFrame;

use super::format_mapper::FrameBridgeError;

/// Ensure that an AudioOutputFrame contains only silent samples (all zero).
///
/// Empty samples are considered silent.
/// Any non-zero sample causes NonSilentFrameRejected.
pub fn ensure_silent_frame(frame: &AudioOutputFrame) -> Result<(), FrameBridgeError> {
    // Check for any non-zero sample
    if frame.samples.iter().any(|&s| s != 0.0) {
        return Err(FrameBridgeError::NonSilentFrameRejected);
    }
    Ok(())
}
