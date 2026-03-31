use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use ringbuf::traits::Consumer;

use crate::audio::errors::AudioResult;

pub(super) struct ConsumerLoopConfig {
    pub(super) device_ch: u16,
    pub(super) started: Instant,
    pub(super) deadline: Instant,
    pub(super) log_every: Duration,
}

pub(super) struct ConsumerLoopRuntime<'a> {
    pub(super) done: &'a Arc<AtomicBool>,
    pub(super) underrun_events: &'a Arc<AtomicU64>,
    pub(super) cfg: ConsumerLoopConfig,
}

pub(super) struct ConsumerLoopCallbacks<'a> {
    pub(super) wait_for_event: &'a mut dyn FnMut(),
    pub(super) get_available_frames: &'a mut dyn FnMut() -> AudioResult<usize>,
    pub(super) render_samples: &'a mut dyn FnMut(usize, &[f32]) -> AudioResult<()>,
    pub(super) log_line: &'a mut dyn FnMut(u64, f64, f64, usize),
}

pub(super) fn run_consumer_loop(
    cons: &mut impl Consumer<Item = f32>,
    runtime: ConsumerLoopRuntime<'_>,
    callbacks: ConsumerLoopCallbacks<'_>,
) -> AudioResult<u64> {
    let mut last_log = Instant::now();
    let mut samples_f32 = vec![0.0f32; runtime.cfg.device_ch as usize];
    let mut total_frames_played: u64 = 0;

    while Instant::now() < runtime.cfg.deadline {
        (callbacks.wait_for_event)();
        let available_frames = (callbacks.get_available_frames)()?;

        if available_frames == 0 {
            if runtime.done.load(Ordering::SeqCst) && cons.occupied_len() == 0 {
                break;
            }
            continue;
        }

        let needed_samples = available_frames.saturating_mul(runtime.cfg.device_ch as usize);
        if needed_samples > samples_f32.len() {
            samples_f32.resize(needed_samples, 0.0);
        }

        let slice = &mut samples_f32[..needed_samples];
        let popped = cons.pop_slice(slice);
        if popped < needed_samples {
            runtime.underrun_events.fetch_add(1, Ordering::Relaxed);
            slice[popped..].fill(0.0);
        }

        (callbacks.render_samples)(available_frames, slice)?;
        total_frames_played = total_frames_played.saturating_add(available_frames as u64);

        if last_log.elapsed() >= runtime.cfg.log_every {
            last_log = Instant::now();
            let elapsed = runtime.cfg.started.elapsed().as_secs_f64().max(0.001);
            let avg_fps = (total_frames_played as f64) / elapsed;
            (callbacks.log_line)(total_frames_played, elapsed, avg_fps, cons.occupied_len());
        }

        if runtime.done.load(Ordering::SeqCst) && cons.occupied_len() == 0 {
            break;
        }
    }

    Ok(total_frames_played)
}
