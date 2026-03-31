//! Audio subsystem diagnostics and monitoring.
//!
//! Provides runtime metrics and health checks for the audio pipeline.

use std::sync::atomic::{AtomicU64, Ordering};

/// Global audio subsystem diagnostics.
pub struct AudioDiagnostics {
    /// Total sessions created since startup
    sessions_created: AtomicU64,
    /// Total sessions successfully completed
    sessions_completed: AtomicU64,
    /// Total sessions that failed
    sessions_failed: AtomicU64,
    /// Peak concurrent sessions
    peak_concurrent_sessions: AtomicU64,
    /// Total decode errors
    decode_errors: AtomicU64,
    /// Total WASAPI errors
    wasapi_errors: AtomicU64,
}

impl Default for AudioDiagnostics {
    fn default() -> Self {
        Self {
            sessions_created: AtomicU64::new(0),
            sessions_completed: AtomicU64::new(0),
            sessions_failed: AtomicU64::new(0),
            peak_concurrent_sessions: AtomicU64::new(0),
            decode_errors: AtomicU64::new(0),
            wasapi_errors: AtomicU64::new(0),
        }
    }
}

impl AudioDiagnostics {
    pub fn record_session_created(&self) {
        self.sessions_created.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_session_completed(&self) {
        self.sessions_completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_session_failed(&self) {
        self.sessions_failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_concurrent_sessions(&self, count: u64) {
        let mut peak = self.peak_concurrent_sessions.load(Ordering::Relaxed);
        while count > peak {
            match self.peak_concurrent_sessions.compare_exchange(
                peak,
                count,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => peak = actual,
            }
        }
    }

    pub fn record_decode_error(&self) {
        self.decode_errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_wasapi_error(&self) {
        self.wasapi_errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn summary(&self) -> AudioDiagnosticsSummary {
        AudioDiagnosticsSummary {
            sessions_created: self.sessions_created.load(Ordering::Relaxed),
            sessions_completed: self.sessions_completed.load(Ordering::Relaxed),
            sessions_failed: self.sessions_failed.load(Ordering::Relaxed),
            peak_concurrent_sessions: self.peak_concurrent_sessions.load(Ordering::Relaxed),
            decode_errors: self.decode_errors.load(Ordering::Relaxed),
            wasapi_errors: self.wasapi_errors.load(Ordering::Relaxed),
        }
    }
}

/// Snapshot of audio diagnostics at a point in time.
#[derive(Debug, Clone)]
pub struct AudioDiagnosticsSummary {
    pub sessions_created: u64,
    pub sessions_completed: u64,
    pub sessions_failed: u64,
    pub peak_concurrent_sessions: u64,
    pub decode_errors: u64,
    pub wasapi_errors: u64,
}

impl AudioDiagnosticsSummary {
    pub fn success_rate(&self) -> f64 {
        if self.sessions_created == 0 {
            return 0.0;
        }
        self.sessions_completed as f64 / self.sessions_created as f64
    }

    pub fn failure_rate(&self) -> f64 {
        if self.sessions_created == 0 {
            return 0.0;
        }
        self.sessions_failed as f64 / self.sessions_created as f64
    }

    pub fn display(&self) -> String {
        format!(
            "Audio Diagnostics: created={}, completed={}, failed={}, peak_concurrent={}, \
             decode_errors={}, wasapi_errors={}, success_rate={:.1}%",
            self.sessions_created,
            self.sessions_completed,
            self.sessions_failed,
            self.peak_concurrent_sessions,
            self.decode_errors,
            self.wasapi_errors,
            self.success_rate() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics() {
        let diag = AudioDiagnostics::default();
        assert_eq!(diag.summary().sessions_created, 0);

        diag.record_session_created();
        diag.record_session_created();
        diag.record_session_completed();
        diag.record_session_failed();

        let summary = diag.summary();
        assert_eq!(summary.sessions_created, 2);
        assert_eq!(summary.sessions_completed, 1);
        assert_eq!(summary.sessions_failed, 1);
        assert!(summary.success_rate() > 0.4 && summary.success_rate() < 0.6);
    }
}
