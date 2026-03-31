use std::{
    f32::consts::PI,
    sync::{atomic::Ordering, Arc},
    thread,
    time::Duration,
};

use crossbeam_channel::TryRecvError;
use ringbuf::traits::Producer;

use crate::audio::smoke::SmokeMode;

use super::WasapiSoakStats;

pub(super) struct ProducerConfig {
    pub(super) mode: SmokeMode,
    pub(super) frequency_hz: f32,
    pub(super) amplitude: f32,
    pub(super) sample_rate: usize,
    pub(super) channels: usize,
    pub(super) ring_capacity_samples: usize,
    pub(super) stats: Arc<WasapiSoakStats>,
    pub(super) stop_rx: crossbeam_channel::Receiver<()>,
}

pub(super) fn spawn_producer(
    cfg: ProducerConfig,
    mut prod: impl Producer<Item = f32> + Send + 'static,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut phase = 0.0f32;
        let phase_inc = 2.0 * PI * cfg.frequency_hz.max(1.0) / (cfg.sample_rate.max(1) as f32);
        let chunk_frames = 512usize;
        let chunk_samples = chunk_frames * cfg.channels;

        loop {
            match cfg.stop_rx.try_recv() {
                Ok(()) => break,
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            }

            if prod.vacant_len() < chunk_samples {
                let fill = cfg.ring_capacity_samples.saturating_sub(prod.vacant_len());
                let _ = cfg.stats.record_ring_write_blocked(fill as u64);
                thread::sleep(Duration::from_millis(2));
                continue;
            }

            for _ in 0..chunk_frames {
                let s = match cfg.mode {
                    SmokeMode::Sine => phase.sin() * cfg.amplitude,
                    SmokeMode::Silence => 0.0,
                };

                phase += phase_inc;
                if phase > 2.0 * PI {
                    phase -= 2.0 * PI;
                }

                for _ in 0..cfg.channels {
                    let _ = prod.try_push(s);
                }
            }

            cfg.stats.ring_write_events.fetch_add(1, Ordering::Relaxed);
            cfg.stats
                .ring_write_samples
                .fetch_add(chunk_samples as u64, Ordering::Relaxed);
        }
    })
}
