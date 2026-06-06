use crate::playback::audio_route_coordinator::AudioRouteCoordinatorError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRouteIntegrationError {
    InvalidConfig(AudioRouteCoordinatorError),
    Coordinator(AudioRouteCoordinatorError),
    Closed,
    NotInitialized,
}
