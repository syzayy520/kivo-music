use super::config::OutputThreadRuntimeConfig;

#[test]
fn default_config_has_valid_timeouts() {
    let config = OutputThreadRuntimeConfig::default();
    assert!(config.has_valid_timeouts());
}

#[test]
fn low_latency_config_uses_lower_latency() {
    let config = OutputThreadRuntimeConfig::low_latency();
    assert_eq!(config.target_latency_ms, 20);
    assert!(config.has_valid_timeouts());
}

#[test]
fn device_reset_can_be_enabled() {
    let config = OutputThreadRuntimeConfig::default().with_device_reset_enabled();
    assert!(config.is_reset_allowed());
}

#[test]
fn reset_allowed_reflects_flag() {
    let config = OutputThreadRuntimeConfig::default();
    assert!(!config.is_reset_allowed());

    let config_with_reset = config.with_device_reset_enabled();
    assert!(config_with_reset.is_reset_allowed());
}

#[test]
fn invalid_timeout_config_is_detected() {
    let mut config = OutputThreadRuntimeConfig::default();
    assert!(config.has_valid_timeouts());

    config.target_latency_ms = 0;
    assert!(!config.has_valid_timeouts());
}
