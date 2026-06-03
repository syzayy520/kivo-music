// output_thread_boundary/report_success_builders.rs
//
// Success report builders for WasapiOutputThreadSmokeReport.

use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for thread spawn failure.
    pub fn thread_spawn_failed(error: String) -> Self {
        Self {
            attempted: true,
            thread_spawn_attempted: true,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for thread recv timeout.
    pub fn thread_recv_timeout(timeout_ms: u64) -> Self {
        Self {
            attempted: true,
            thread_spawn_attempted: true,
            thread_spawned: true,
            thread_report_recv_attempted: true,
            thread_recv_timeout_ms: Some(timeout_ms),
            thread_recv_timed_out: true,
            ..Default::default()
        }
    }

    /// Create a report for thread join failure.
    pub fn thread_join_failed(error: String) -> Self {
        Self {
            attempted: true,
            thread_spawn_attempted: true,
            thread_spawned: true,
            thread_report_recv_attempted: true,
            thread_report_received: true,
            thread_join_attempted: true,
            thread_join_failed: true,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for thread panic.
    pub fn thread_panic(panic_message: String) -> Self {
        Self {
            attempted: true,
            thread_spawn_attempted: true,
            thread_spawned: true,
            thread_report_recv_attempted: true,
            thread_report_received: true,
            thread_join_attempted: true,
            thread_joined: true,
            thread_panic_caught: true,
            thread_panic_message: Some(panic_message),
            ..Default::default()
        }
    }
}
