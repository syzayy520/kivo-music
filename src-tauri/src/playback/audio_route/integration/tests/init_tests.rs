use crate::playback::audio_route::AudioRouteError;
use crate::playback::audio_route_coordinator::AudioRouteCoordinatorError;
use crate::playback::audio_route_integration::{
    AudioRouteIntegration, AudioRouteIntegrationConfig, AudioRouteIntegrationError,
};

use super::fixtures::{config, float_stream};

#[test]
fn integration_creates_coordinator_from_valid_config() -> Result<(), String> {
    let integration =
        AudioRouteIntegration::new(config(8)).map_err(|error| format!("{error:?}"))?;
    let report = integration.report();

    assert!(report.initialized);
    assert!(report.coordinator_initialized);
    assert_eq!(report.capacity_frames, 8);
    assert!(!report.closed);
    Ok(())
}

#[test]
fn config_new_reuses_coordinator_validation() {
    assert!(matches!(
        AudioRouteIntegrationConfig::new(float_stream(), 0),
        Err(AudioRouteIntegrationError::InvalidConfig(
            AudioRouteCoordinatorError::InvalidConfig(AudioRouteError::InvalidCapacity)
        ))
    ));
}

#[test]
fn invalid_config_maps_to_invalid_config() {
    let config = AudioRouteIntegrationConfig {
        coordinator: crate::playback::audio_route_coordinator::AudioRouteCoordinatorConfig {
            route: crate::playback::audio_route::AudioRouteConfig {
                stream: float_stream(),
                capacity_frames: 0,
            },
        },
    };

    assert!(matches!(
        AudioRouteIntegration::new(config),
        Err(AudioRouteIntegrationError::InvalidConfig(
            AudioRouteCoordinatorError::InvalidConfig(AudioRouteError::InvalidCapacity)
        ))
    ));
}
