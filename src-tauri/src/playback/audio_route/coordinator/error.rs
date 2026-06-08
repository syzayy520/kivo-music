use crate::playback::audio_route::AudioRouteError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRouteCoordinatorError {
    InvalidConfig(AudioRouteError),
    Route(AudioRouteError),
    Closed,
    NotInitialized,
}
