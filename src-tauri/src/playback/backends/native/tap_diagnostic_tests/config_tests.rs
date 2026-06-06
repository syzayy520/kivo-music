use crate::playback::native_pipeline_route_tap_diagnostic_policy::{
    NativeTapDiagnosticConfig, NativeTapDiagnosticPolicyError,
};

use super::super::KivoNativeEngine;

#[test]
fn valid_explicit_config_enables_diagnostic_sidecar() {
    let config = NativeTapDiagnosticConfig::new(32).expect("valid diagnostic capacity");
    let engine = KivoNativeEngine::new_with_tap_diagnostic_policy(config);

    assert!(engine.tap_diagnostic.policy().is_some());
}

#[test]
fn zero_capacity_is_rejected_without_enabling_sidecar() {
    let engine = KivoNativeEngine::new();
    let result = NativeTapDiagnosticConfig::new(0);

    assert_eq!(
        result,
        Err(NativeTapDiagnosticPolicyError::InvalidDiagnosticRouteCapacityFrames)
    );
    assert!(engine.tap_diagnostic.policy().is_none());
}

#[test]
fn config_keeps_diagnostic_route_capacity_name() -> Result<(), String> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/playback/native_pipeline_route_tap_diagnostic_policy/config.rs");
    let source = std::fs::read_to_string(path).map_err(|error| error.to_string())?;

    assert!(source.contains("diagnostic_route_capacity_frames"));
    assert!(!source.contains("output_capacity"));
    Ok(())
}
