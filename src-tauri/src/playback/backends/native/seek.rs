use super::super::native_unsupported::unsupported_operation;
use super::KivoNativeEngine;
use crate::playback::errors::PlaybackResult;
use crate::playback::state::PlaybackState;

pub(super) fn seek_track(
    _engine: &mut KivoNativeEngine,
    _position_ms: u64,
) -> PlaybackResult<PlaybackState> {
    unsupported_operation("native playback seek")
}
