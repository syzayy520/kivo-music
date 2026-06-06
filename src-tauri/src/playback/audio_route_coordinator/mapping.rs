use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_route::{AudioRouteError, AudioRouteReport};

use super::error::AudioRouteCoordinatorError;
use super::report::AudioRouteCoordinatorReport;
use super::state::AudioRouteCoordinatorState;

pub(super) fn map_config_error(error: AudioRouteError) -> AudioRouteCoordinatorError {
    AudioRouteCoordinatorError::InvalidConfig(error)
}

pub(super) fn map_route_error(error: AudioRouteError) -> AudioRouteCoordinatorError {
    match error {
        AudioRouteError::Closed => AudioRouteCoordinatorError::Closed,
        other => AudioRouteCoordinatorError::Route(other),
    }
}

pub(super) fn route_error_is_backpressure(error: &AudioRouteError) -> bool {
    matches!(
        error,
        AudioRouteError::Bridge(SourceToRingBufferBridgeError::BufferFull)
    )
}

pub(super) fn report_from_route(
    route: &AudioRouteReport,
    state: &AudioRouteCoordinatorState,
    closed: bool,
) -> AudioRouteCoordinatorReport {
    AudioRouteCoordinatorReport {
        initialized: true,
        route_initialized: route.initialized,
        closed,
        capacity_frames: route.capacity_frames,
        pending_frames: route.pending_frames,
        total_requested_frames: route.total_requested_frames,
        total_accepted_frames: route.total_accepted_frames,
        total_rejected_frames: route.total_rejected_frames,
        feed_count: state.feed_count,
        backpressure_count: state.backpressure_count,
        partial_write_count: state.partial_write_count,
        source_closed_seen: state.source_closed_seen,
    }
}
