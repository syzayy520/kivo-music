use super::control;
use super::KivoNativeEngine;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackStatus;

pub(super) fn seek_track(
    engine: &mut KivoNativeEngine,
    position_ms: u64,
) -> PlaybackResult<PlaybackState> {
    if engine.state.current_track.is_none() {
        return Err(PlaybackError::NoTrack(
            "seek requires a loaded track".to_string(),
        ));
    }

    match engine.state.status {
        PlaybackStatus::Idle => {}
        PlaybackStatus::Paused => {
            return Err(PlaybackError::InvalidControlState(
                "seek while paused requires output flush contract".to_string(),
            ));
        }
        PlaybackStatus::Playing => {
            return Err(PlaybackError::InvalidControlState(
                "seek while playing requires output flush contract".to_string(),
            ));
        }
        PlaybackStatus::Loading => {
            return Err(PlaybackError::InvalidControlState(
                "seek while loading".to_string(),
            ));
        }
        PlaybackStatus::Stopped => {
            return Err(PlaybackError::InvalidControlState(
                "seek while stopped".to_string(),
            ));
        }
        PlaybackStatus::Failed => {
            return Err(PlaybackError::InvalidControlState(
                "seek while failed".to_string(),
            ));
        }
    }

    if let Some(duration_ms) = engine.state.timeline.duration_ms {
        if position_ms > duration_ms {
            return Err(PlaybackError::SeekOutOfRange {
                position_ms,
                duration_ms,
            });
        }
    }

    match engine.pipeline.seek_decoder(position_ms) {
        Ok(()) => {
            engine.state.timeline.position_ms = position_ms;
            engine.state.error = None;
            Ok(engine.state.clone())
        }
        Err(_) => control::record_error(
            engine,
            PlaybackError::SeekTransactionFailed(
                "native pipeline seek transaction failed".to_string(),
            ),
        ),
    }
}
