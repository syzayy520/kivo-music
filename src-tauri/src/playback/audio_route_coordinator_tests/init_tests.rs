use crate::playback::audio_route::AudioRouteError;
use crate::playback::audio_route_coordinator::{
    AudioRouteCoordinator, AudioRouteCoordinatorConfig, AudioRouteCoordinatorError,
};

use super::fixtures::{config, float_stream};

#[test]
fn coordinator_creates_route_from_valid_config() -> Result<(), String> {
    let coordinator =
        AudioRouteCoordinator::new(config(8)).map_err(|error| format!("{error:?}"))?;
    let report = coordinator.report();

    assert!(report.initialized);
    assert!(report.route_initialized);
    assert_eq!(report.capacity_frames, 8);
    assert!(!report.closed);
    Ok(())
}

#[test]
fn config_new_reuses_route_config_validation() {
    assert!(matches!(
        AudioRouteCoordinatorConfig::new(float_stream(), 0),
        Err(AudioRouteCoordinatorError::InvalidConfig(
            AudioRouteError::InvalidCapacity
        ))
    ));
}

#[test]
fn invalid_route_config_maps_to_invalid_config() {
    let config = AudioRouteCoordinatorConfig {
        route: crate::playback::audio_route::AudioRouteConfig {
            stream: float_stream(),
            capacity_frames: 0,
        },
    };

    assert!(matches!(
        AudioRouteCoordinator::new(config),
        Err(AudioRouteCoordinatorError::InvalidConfig(
            AudioRouteError::InvalidCapacity
        ))
    ));
}
