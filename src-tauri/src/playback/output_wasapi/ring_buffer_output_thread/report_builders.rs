use super::report::WasapiRingBufferOutputThreadSmokeReport;

impl WasapiRingBufferOutputThreadSmokeReport {
    pub fn skipped_env_missing() -> Self {
        Self {
            attempted: false,
            skipped: true,
            skipped_reason: Some("env opt-in not enabled"),
            ..Default::default()
        }
    }

    pub fn skipped_non_windows() -> Self {
        Self {
            attempted: false,
            skipped: true,
            skipped_reason: Some("non-windows platform"),
            ..Default::default()
        }
    }

    pub fn scaffold_ready_report() -> Self {
        Self {
            attempted: true,
            skipped: false,
            ring_buffer_created: false,
            output_thread_spawned: false,
            ..Default::default()
        }
    }
}
