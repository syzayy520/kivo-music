use super::super::failure::OutputFlushFailure;

#[test]
fn failure_route_closed_is_fatal() {
    let f = OutputFlushFailure::RouteClosed("closed".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}

#[test]
fn failure_sink_failure_is_data_loss() {
    let f = OutputFlushFailure::SinkFailure("error".into());
    assert!(f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_backpressure_is_retryable() {
    let f = OutputFlushFailure::Backpressure("full".into());
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_unsupported_target_is_fatal() {
    let f = OutputFlushFailure::UnsupportedTarget("unsupported".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}

#[test]
fn failure_stale_generation_is_retryable() {
    let f = OutputFlushFailure::StaleGeneration {
        expected_generation: 10,
        actual_generation: 9,
    };
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_render_thread_failure_is_retryable() {
    let f = OutputFlushFailure::RenderThreadFailure("timeout".into());
    assert!(!f.is_data_loss());
    assert!(f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_device_reset_failure_is_data_loss() {
    let f = OutputFlushFailure::DeviceResetFailure("reset failed".into());
    assert!(f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(!f.is_fatal());
}

#[test]
fn failure_unknown_is_fatal() {
    let f = OutputFlushFailure::Unknown("unknown".into());
    assert!(!f.is_data_loss());
    assert!(!f.is_retryable());
    assert!(f.is_fatal());
}
