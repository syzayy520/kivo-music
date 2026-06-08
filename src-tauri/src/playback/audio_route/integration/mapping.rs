use crate::playback::audio_route_coordinator::{
    AudioRouteCoordinatorError, AudioRouteCoordinatorReport,
};

use super::error::AudioRouteIntegrationError;
use super::report::AudioRouteIntegrationReport;
use super::state::AudioRouteIntegrationState;

pub(super) fn map_config_error(error: AudioRouteCoordinatorError) -> AudioRouteIntegrationError {
    AudioRouteIntegrationError::InvalidConfig(error)
}

pub(super) fn map_coordinator_error(
    error: AudioRouteCoordinatorError,
) -> AudioRouteIntegrationError {
    match error {
        AudioRouteCoordinatorError::Closed => AudioRouteIntegrationError::Closed,
        other => AudioRouteIntegrationError::Coordinator(other),
    }
}

pub(super) fn report_from_coordinator(
    coordinator: &AudioRouteCoordinatorReport,
    state: &AudioRouteIntegrationState,
    closed: bool,
) -> AudioRouteIntegrationReport {
    AudioRouteIntegrationReport {
        initialized: true,
        coordinator_initialized: coordinator.initialized && coordinator.route_initialized,
        closed,
        pending_frames: coordinator.pending_frames,
        capacity_frames: coordinator.capacity_frames,
        total_requested_frames: coordinator.total_requested_frames,
        total_accepted_frames: coordinator.total_accepted_frames,
        total_rejected_frames: coordinator.total_rejected_frames,
        input_count: state.input_count,
        backpressure_count: coordinator.backpressure_count,
        partial_write_count: coordinator.partial_write_count,
        source_closed_seen: coordinator.source_closed_seen,
    }
}
