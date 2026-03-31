use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex, MutexGuard,
};

use serde::{Deserialize, Serialize};

/// A minimal, stable, and serde-serializable snapshot of [`WasapiSoakStats`].
///
/// This is intended to be a long-lived contract between backend and any
/// future status/evidence reporting layers. Keep field names stable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(dead_code)]
pub struct WasapiSoakStatsSnapshot {
    pub underrun_events: u64,
    pub underrun_samples: u64,
    pub underrun_recoveries: u64,

    pub ring_write_events: u64,
    pub ring_write_blocked_events: u64,

    pub last_error: Option<String>,
}

#[derive(Default)]
pub struct WasapiSoakStats {
    pub underrun_events: AtomicU64,
    pub underrun_samples: AtomicU64,
    pub underrun_recoveries: AtomicU64,
    pub last_underrun_ring_fill_samples: AtomicU64,

    pub ring_write_samples: AtomicU64,
    pub ring_write_events: AtomicU64,
    pub ring_write_blocked_events: AtomicU64,
    pub last_producer_blocked_ring_fill_samples: AtomicU64,

    pub ring_discard_samples: AtomicU64,

    pub overrun_events: AtomicU64,
    pub overrun_samples: AtomicU64,
    pub last_overrun_ring_fill_samples: AtomicU64,

    pub device_rebuilds: AtomicU64,

    last_error: Mutex<Option<String>>,
}

impl WasapiSoakStats {
    /// Record an error message to help diagnose output failures.
    pub fn set_last_error(&self, msg: String) {
        *lock_poison(&self.last_error) = Some(msg);
    }

    pub fn last_error(&self) -> Option<String> {
        lock_poison(&self.last_error).clone()
    }

    /// Take a minimal, stable snapshot for serialization/reporting.
    #[allow(dead_code)]
    pub fn snapshot(&self) -> WasapiSoakStatsSnapshot {
        WasapiSoakStatsSnapshot {
            underrun_events: self.underrun_events.load(Ordering::Relaxed),
            underrun_samples: self.underrun_samples.load(Ordering::Relaxed),
            underrun_recoveries: self.underrun_recoveries.load(Ordering::Relaxed),
            ring_write_events: self.ring_write_events.load(Ordering::Relaxed),
            ring_write_blocked_events: self.ring_write_blocked_events.load(Ordering::Relaxed),
            last_error: self.last_error(),
        }
    }

    pub fn record_ring_write_blocked(&self, ring_fill_samples: u64) -> u64 {
        self.last_producer_blocked_ring_fill_samples
            .store(ring_fill_samples, Ordering::Relaxed);
        self.ring_write_blocked_events
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1)
    }

    /// Count a recovery attempt after an underrun. This is used to observe how often we
    /// had to transition from padded silence back to real audio.
    pub fn record_underrun_recovery(&self) -> u64 {
        self.underrun_recoveries
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1)
    }

    pub fn record_underrun(&self, missing_samples: u64, ring_fill_samples: u64) -> u64 {
        self.last_underrun_ring_fill_samples
            .store(ring_fill_samples, Ordering::Relaxed);
        self.underrun_samples
            .fetch_add(missing_samples, Ordering::Relaxed);
        self.underrun_events
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1)
    }
}

fn lock_poison<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    match m.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn snapshot_default_does_not_panic() {
        let stats = WasapiSoakStats::default();
        let snap = stats.snapshot();
        assert_eq!(snap.underrun_events, 0);
        assert_eq!(snap.underrun_samples, 0);
        assert_eq!(snap.underrun_recoveries, 0);
        assert_eq!(snap.ring_write_events, 0);
        assert_eq!(snap.ring_write_blocked_events, 0);
        assert_eq!(snap.last_error, None);
    }

    #[test]
    fn poisoned_last_error_lock_does_not_panic() {
        let stats = Arc::new(WasapiSoakStats::default());
        let stats2 = stats.clone();
        let _ = std::thread::spawn(move || {
            let _guard = lock_poison(&stats2.last_error);
            panic!("intentional poison");
        })
        .join();

        stats.set_last_error("recoverable".to_string());
        assert_eq!(stats.last_error().as_deref(), Some("recoverable"));
    }
}
