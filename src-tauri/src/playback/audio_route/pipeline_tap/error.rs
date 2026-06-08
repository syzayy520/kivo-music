use crate::playback::audio_route_integration::AudioRouteIntegrationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRoutePipelineTapError {
    InvalidConfig(AudioRouteIntegrationError),
    Integration(AudioRouteIntegrationError),
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioRoutePipelineTapErrorKind {
    InvalidConfig,
    IntegrationClosed,
    IntegrationFailed,
}
