use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use ringbuf::traits::Consumer;

use crate::audio::{diag::DiagWriter, error::AudioResult};

use super::{
    pack::{bytes_per_sample, pack_samples_into},
    wasapi_shared_consumer_diag::{LogState, PeriodicLogInput, RecoveryRamp},
    WasapiSoakStats,
};

pub(super) struct ConsumerSummary {
    pub(super) total_frames_written: u64,
    pub(super) total_samples_written: u64,
    pub(super) total_samples_popped: u64,
    pub(super) total_samples_padded: u64,
    pub(super) ring_watermark_max: usize,
}

pub(super) struct ConsumerConfig {
    pub(super) deadline: Instant,
    pub(super) started: Instant,
    pub(super) log_every: Duration,
    pub(super) buffer_frames: usize,
    pub(super) channels: usize,
    pub(super) ring_capacity_samples: usize,
    pub(super) bits_per_sample: usize,
    pub(super) valid_bits: usize,
    pub(super) sample_type: wasapi::SampleType,
    pub(super) stats: Arc<WasapiSoakStats>,
}

pub(super) fn run_consumer_loop(
    cons: &mut impl Consumer<Item = f32>,
    diag: &mut DiagWriter,
    cfg: ConsumerConfig,
    mut wait_for_event: impl FnMut(),
    mut get_available_frames: impl FnMut() -> AudioResult<usize>,
    mut write_to_device: impl FnMut(usize, &[u8]) -> AudioResult<()>,
) -> AudioResult<ConsumerSummary> {
    let max_frames_per_write = cfg.buffer_frames.max(1);
    let max_samples_per_write = max_frames_per_write.saturating_mul(cfg.channels).max(1);
    let mut samples_f32 = vec![0.0f32; max_samples_per_write];
    let bytes_per_samp = bytes_per_sample(cfg.sample_type, cfg.bits_per_sample);
    let mut packed = Vec::<u8>::with_capacity(max_samples_per_write * bytes_per_samp);

    let mut total_frames_written: u64 = 0;
    let mut total_samples_written: u64 = 0;
    let mut total_samples_popped: u64 = 0;
    let mut total_samples_padded: u64 = 0;
    let mut ring_watermark_max: usize = 0;

    let mut log_state = LogState::default();
    let mut last_log = Instant::now();
    let mut recovery_ramp = RecoveryRamp::default();

    while Instant::now() < cfg.deadline {
        wait_for_event();

        let available_frames = get_available_frames()?;
        if available_frames == 0 {
            continue;
        }

        let needed_samples = available_frames.saturating_mul(cfg.channels);
        if needed_samples > samples_f32.len() {
            samples_f32.resize(needed_samples, 0.0);
        }

        let ring_fill_before = cons.occupied_len();
        ring_watermark_max = ring_watermark_max.max(ring_fill_before);

        let slice = &mut samples_f32[..needed_samples];
        let popped = cons.pop_slice(slice);
        let mut did_pad_silence = false;

        if popped < needed_samples {
            did_pad_silence = true;
            let missing = (needed_samples - popped) as u64;
            let e = cfg.stats.record_underrun(missing, ring_fill_before as u64);
            cfg.stats.set_last_error(format!(
                "UNDERRUN missing_samples={missing} ring_fill_before_samples={ring_fill_before} needed_samples={needed_samples}"
            ));
            slice[popped..].fill(0.0);

            recovery_ramp.on_underrun();
            let _ = cfg.stats.record_underrun_recovery();

            if e <= 3 || e.is_multiple_of(100) {
                diag.line(format!(
                    "wasapi_shared: UNDERRUN event={} missing_samples={} ring_fill_before_samples={} needed_samples={}",
                    e, missing, ring_fill_before, needed_samples
                ))?;
            }
        }

        recovery_ramp.apply_after_recovery(diag, popped, did_pad_silence, slice)?;

        pack_samples_into(
            &mut packed,
            slice,
            cfg.sample_type,
            cfg.bits_per_sample,
            cfg.valid_bits,
        );
        write_to_device(available_frames, &packed)?;

        total_frames_written = total_frames_written.saturating_add(available_frames as u64);
        total_samples_written = total_samples_written.saturating_add(needed_samples as u64);
        total_samples_popped = total_samples_popped.saturating_add(popped as u64);
        total_samples_padded =
            total_samples_padded.saturating_add((needed_samples - popped) as u64);

        if last_log.elapsed() >= cfg.log_every {
            last_log = Instant::now();

            let fill = cons.occupied_len();
            log_state.emit_periodic(PeriodicLogInput {
                diag,
                stats: &cfg.stats,
                started: cfg.started,
                available_frames,
                fill,
                ring_capacity_samples: cfg.ring_capacity_samples,
                total_frames_written,
                total_samples_popped,
                total_samples_padded,
            })?;
        }
    }

    Ok(ConsumerSummary {
        total_frames_written,
        total_samples_written,
        total_samples_popped,
        total_samples_padded,
        ring_watermark_max,
    })
}
