use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
};

use crate::audio::errors::{AudioError, AudioResult};

/// A simple blocking ring buffer for interleaved f32 samples.
///
/// - Producer writes with `write_blocking`.
/// - Consumer reads with `read_blocking`.
///
/// P0 goal: correctness + diagnosability > perfect real-time behavior.
#[derive(Clone)]
pub struct PcmRingBuffer {
    inner: Arc<Inner>,
}

struct Inner {
    buf: Mutex<VecDeque<f32>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
    closed: AtomicBool,
}

impl PcmRingBuffer {
    pub fn new(capacity_samples: usize) -> Self {
        let cap = capacity_samples.max(1024);
        Self {
            inner: Arc::new(Inner {
                buf: Mutex::new(VecDeque::with_capacity(cap)),
                not_empty: Condvar::new(),
                not_full: Condvar::new(),
                capacity: cap,
                closed: AtomicBool::new(false),
            }),
        }
    }

    pub fn close(&self) {
        self.inner.closed.store(true, Ordering::SeqCst);
        self.inner.not_empty.notify_all();
        self.inner.not_full.notify_all();
    }

    pub fn write_blocking(&self, samples: &[f32]) -> AudioResult<()> {
        let mut offset = 0;
        while offset < samples.len() {
            if self.inner.closed.load(Ordering::SeqCst) {
                return Err(AudioError::RingClosed);
            }

            let mut guard = self.inner.buf.lock().map_err(|_| AudioError::RingClosed)?;
            while guard.len() >= self.inner.capacity {
                if self.inner.closed.load(Ordering::SeqCst) {
                    return Err(AudioError::RingClosed);
                }
                guard = self
                    .inner
                    .not_full
                    .wait(guard)
                    .map_err(|_| AudioError::RingClosed)?;
            }

            let room = self.inner.capacity - guard.len();
            let n = room.min(samples.len() - offset);
            guard.extend(&samples[offset..offset + n]);
            offset += n;

            self.inner.not_empty.notify_one();
        }
        Ok(())
    }

    /// Read up to `out.len()` samples.
    /// Returns 0 when closed and drained.
    pub fn read_blocking(&self, out: &mut [f32]) -> AudioResult<usize> {
        let mut guard = self.inner.buf.lock().map_err(|_| AudioError::RingClosed)?;

        while guard.is_empty() {
            if self.inner.closed.load(Ordering::SeqCst) {
                return Ok(0);
            }
            guard = self
                .inner
                .not_empty
                .wait(guard)
                .map_err(|_| AudioError::RingClosed)?;
        }

        let n = out.len().min(guard.len());
        for slot in out.iter_mut().take(n) {
            *slot = guard.pop_front().unwrap_or(0.0);
        }
        self.inner.not_full.notify_one();
        Ok(n)
    }
}
