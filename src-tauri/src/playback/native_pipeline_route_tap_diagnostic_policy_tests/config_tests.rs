use crate::playback::native_pipeline_route_tap_diagnostic_policy::{
    NativeTapDiagnosticConfig, NativeTapDiagnosticPolicyError,
};

#[test]
fn positive_diagnostic_route_capacity_is_accepted() -> Result<(), String> {
    let config = NativeTapDiagnosticConfig::new(32).map_err(|error| format!("{error:?}"))?;

    assert_eq!(config.diagnostic_route_capacity_frames, 32);
    Ok(())
}

#[test]
fn zero_diagnostic_route_capacity_is_rejected() {
    assert_eq!(
        NativeTapDiagnosticConfig::new(0),
        Err(NativeTapDiagnosticPolicyError::InvalidDiagnosticRouteCapacityFrames)
    );
}

#[test]
fn config_names_capacity_as_diagnostic_route_frames() -> Result<(), String> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/playback/native_pipeline_route_tap_diagnostic_policy/config.rs");
    let source = std::fs::read_to_string(path).map_err(|error| format!("{error}"))?;

    assert!(source.contains("diagnostic_route_capacity_frames"));
    assert!(!source.contains("wasapi"));
    assert!(!source.contains("latency"));
    assert!(!source.contains("byte_count"));
    Ok(())
}
