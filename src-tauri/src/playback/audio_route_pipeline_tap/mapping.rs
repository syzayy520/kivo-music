use crate::playback::audio_route_integration::{
    AudioRouteIntegrationError, AudioRouteIntegrationReport,
};

use super::error::{AudioRoutePipelineTapError, AudioRoutePipelineTapErrorKind};
use super::frame_kind::AudioRouteTapFrameKind;
use super::report::AudioRoutePipelineTapReport;

pub(super) fn map_integration_error(
    error: AudioRouteIntegrationError,
) -> AudioRoutePipelineTapError {
    match error {
        AudioRouteIntegrationError::Closed => AudioRoutePipelineTapError::Closed,
        other => AudioRoutePipelineTapError::Integration(other),
    }
}

pub(super) fn error_kind(error: &AudioRouteIntegrationError) -> AudioRoutePipelineTapErrorKind {
    match error {
        AudioRouteIntegrationError::InvalidConfig(_) => {
            AudioRoutePipelineTapErrorKind::InvalidConfig
        }
        AudioRouteIntegrationError::Closed => AudioRoutePipelineTapErrorKind::IntegrationClosed,
        AudioRouteIntegrationError::Coordinator(_) | AudioRouteIntegrationError::NotInitialized => {
            AudioRoutePipelineTapErrorKind::IntegrationFailed
        }
    }
}

pub(super) fn report_from_integration(
    integration: &AudioRouteIntegrationReport,
    frame_kind: AudioRouteTapFrameKind,
    last_error: Option<AudioRoutePipelineTapErrorKind>,
) -> AudioRoutePipelineTapReport {
    AudioRoutePipelineTapReport {
        initialized: integration.initialized && integration.coordinator_initialized,
        closed: integration.closed,
        frame_kind,
        input_count: integration.input_count,
        total_accepted_frames: integration.total_accepted_frames,
        total_rejected_frames: integration.total_rejected_frames,
        backpressure_count: integration.backpressure_count,
        partial_write_count: integration.partial_write_count,
        source_closed_seen: integration.source_closed_seen,
        pending_frames: integration.pending_frames,
        capacity_frames: integration.capacity_frames,
        last_error,
    }
}
