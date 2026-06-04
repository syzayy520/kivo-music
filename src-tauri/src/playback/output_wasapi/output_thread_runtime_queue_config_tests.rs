use super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;

#[test]
fn default_config_accepts_zero_pending() {
    let config = OutputThreadRuntimeQueueConfig::default();
    assert!(config.accepts_pending_count(0));
}

#[test]
fn default_config_rejects_at_capacity() {
    let config = OutputThreadRuntimeQueueConfig::default();
    assert!(!config.accepts_pending_count(32));
}

#[test]
fn strict_config_has_lower_capacity() {
    let config = OutputThreadRuntimeQueueConfig::strict();
    assert_eq!(config.max_pending_commands, 8);
    assert!(config.accepts_pending_count(7));
    assert!(!config.accepts_pending_count(8));
}

#[test]
fn permissive_config_has_larger_capacity() {
    let config = OutputThreadRuntimeQueueConfig::permissive();
    assert_eq!(config.max_pending_commands, 64);
    assert!(config.accepts_pending_count(63));
}

#[test]
fn reset_policy_reflects_config() {
    let strict = OutputThreadRuntimeQueueConfig::strict();
    assert!(!strict.allow_reset_when_stopped);

    let permissive = OutputThreadRuntimeQueueConfig::permissive();
    assert!(permissive.allow_reset_when_stopped);

    let default = OutputThreadRuntimeQueueConfig::default();
    assert!(default.allow_reset_when_stopped);
}
