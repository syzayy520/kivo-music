use std::{sync::atomic::Ordering, time::Instant};

use crate::audio::{diag::DiagWriter, error::AudioResult};

use super::WasapiSoakStats;

const RECOVERY_RAMP_SAMPLES: usize = 4096;

fn apply_ramp_in(samples: &mut [f32], total_ramp_samples: usize, ramp_remaining: &mut usize) {
    if samples.is_empty() || total_ramp_samples == 0 || *ramp_remaining == 0 {
        return;
    }
    let already_done = total_ramp_samples.saturating_sub(*ramp_remaining);
    for (i, s) in samples.iter_mut().enumerate() {
        let idx = already_done + i;
        let gain = ((idx + 1) as f32) / (total_ramp_samples as f32);
        *s *= gain;
    }
    *ramp_remaining = ramp_remaining.saturating_sub(samples.len());
}

#[derive(Default)]
pub(super) struct RecoveryRamp {
    ramp_remaining: usize,
    started: bool,
}

impl RecoveryRamp {
    pub(super) fn on_underrun(&mut self) {
        self.ramp_remaining = RECOVERY_RAMP_SAMPLES;
        self.started = false;
    }

    pub(super) fn apply_after_recovery(
        &mut self,
        diag: &mut DiagWriter,
        popped: usize,
        did_pad_silence: bool,
        slice: &mut [f32],
    ) -> AudioResult<()> {
        if self.ramp_remaining == 0 || popped == 0 || did_pad_silence {
            return Ok(());
        }

        if !self.started {
            self.started = true;
            diag.line(format!(
                "wasapi_shared: ramp_in recovery start ramp_samples={}",
                RECOVERY_RAMP_SAMPLES
            ))?;
        }
        let apply_n = popped.min(self.ramp_remaining);
        apply_ramp_in(
            &mut slice[..apply_n],
            RECOVERY_RAMP_SAMPLES,
            &mut self.ramp_remaining,
        );
        if self.ramp_remaining == 0 {
            self.started = false;
        }
        Ok(())
    }
}

#[derive(Default)]
pub(super) struct LogState {
    pub(super) last_total_frames: u64,
    pub(super) last_total_samples_popped: u64,
    pub(super) last_total_samples_padded: u64,
    pub(super) last_underrun_events: u64,
    pub(super) last_blocked_events: u64,
}

pub(super) struct PeriodicLogInput<'a> {
    pub(super) diag: &'a mut DiagWriter,
    pub(super) stats: &'a WasapiSoakStats,
    pub(super) started: Instant,
    pub(super) available_frames: usize,
    pub(super) fill: usize,
    pub(super) ring_capacity_samples: usize,
    pub(super) total_frames_written: u64,
    pub(super) total_samples_popped: u64,
    pub(super) total_samples_padded: u64,
}

impl LogState {
    pub(super) fn emit_periodic(&mut self, input: PeriodicLogInput<'_>) -> AudioResult<()> {
        let fill_pct = if input.ring_capacity_samples == 0 {
            0.0
        } else {
            (input.fill as f32) / (input.ring_capacity_samples as f32)
        };

        let d_frames = input
            .total_frames_written
            .saturating_sub(self.last_total_frames);
        let d_popped = input
            .total_samples_popped
            .saturating_sub(self.last_total_samples_popped);
        let d_padded = input
            .total_samples_padded
            .saturating_sub(self.last_total_samples_padded);
        self.last_total_frames = input.total_frames_written;
        self.last_total_samples_popped = input.total_samples_popped;
        self.last_total_samples_padded = input.total_samples_padded;

        let u_events = input.stats.underrun_events.load(Ordering::Relaxed);
        let u_samples = input.stats.underrun_samples.load(Ordering::Relaxed);
        let u_recoveries = input.stats.underrun_recoveries.load(Ordering::Relaxed);
        let last_u_fill = input
            .stats
            .last_underrun_ring_fill_samples
            .load(Ordering::Relaxed);

        let blocked_events = input
            .stats
            .ring_write_blocked_events
            .load(Ordering::Relaxed);
        let blocked_last_fill = input
            .stats
            .last_producer_blocked_ring_fill_samples
            .load(Ordering::Relaxed);

        if blocked_events != self.last_blocked_events {
            let delta = blocked_events.saturating_sub(self.last_blocked_events);
            self.last_blocked_events = blocked_events;
            if delta > 0 {
                input.diag.line(format!(
                    "wasapi_shared: PRODUCER_BACKPRESSURE +{} blocked_events last_ring_fill_samples={} ring_capacity_samples={}",
                    delta, blocked_last_fill, input.ring_capacity_samples
                ))?;
            }
        }

        if u_events != self.last_underrun_events {
            let delta = u_events.saturating_sub(self.last_underrun_events);
            self.last_underrun_events = u_events;
            if delta > 0 {
                input.diag.line(format!(
                    "wasapi_shared: UNDERRUN_AGG +{} events total_events={} total_missing_samples={} total_recoveries={} last_event_ring_fill_samples={}",
                    delta, u_events, u_samples, u_recoveries, last_u_fill
                ))?;
            }
        }

        input.diag.line(format!(
            "wasapi_shared: t_ms={} avail_frames={} ring_fill={}/{} ({:.0}%) chunk_frames={} chunk_popped_samples={} chunk_padded_samples={} underrun_events={} underrun_samples={} producer_write_samples={} producer_blocked_events={} device_rebuilds={}",
            input.started.elapsed().as_millis(),
            input.available_frames,
            input.fill,
            input.ring_capacity_samples,
            fill_pct * 100.0,
            d_frames,
            d_popped,
            d_padded,
            u_events,
            u_samples,
            input.stats.ring_write_samples.load(Ordering::Relaxed),
            blocked_events,
            input.stats.device_rebuilds.load(Ordering::Relaxed),
        ))?;

        Ok(())
    }
}
