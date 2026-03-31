use std::sync::{atomic::Ordering, Arc};

use crate::audio::{
    diag::DiagWriter,
    error::AudioResult,
    smoke::{SmokeMode, SmokeTestSummary},
};

use super::{wasapi_shared_consumer::ConsumerSummary, WasapiSoakStats};

#[allow(clippy::too_many_arguments)]
pub(super) fn build_smoke_summary(
    config_mode: SmokeMode,
    diag: &DiagWriter,
    device_name: String,
    device_id: String,
    sample_rate: usize,
    channels: usize,
    buffer_frames: usize,
    ring_capacity_samples: usize,
    bits_per_sample: usize,
    sample_type: wasapi::SampleType,
    consumer_summary: &ConsumerSummary,
    stats: &Arc<WasapiSoakStats>,
) -> SmokeTestSummary {
    SmokeTestSummary {
        mode: match config_mode {
            SmokeMode::Silence => "silence",
            SmokeMode::Sine => "sine",
        }
        .to_string(),
        duration_secs: 0.0,
        log_path: Some(diag.path().display().to_string()),
        device_name,
        device_id,
        sample_rate: u32::try_from(sample_rate).unwrap_or(u32::MAX),
        channels: u16::try_from(channels).unwrap_or(u16::MAX),
        wasapi_buffer_frames: u32::try_from(buffer_frames).unwrap_or(u32::MAX),
        ring_capacity_samples: u32::try_from(ring_capacity_samples).unwrap_or(u32::MAX),
        bits_per_sample: u16::try_from(bits_per_sample).unwrap_or(u16::MAX),
        sample_type: format!("{sample_type:?}"),
        total_frames_written: consumer_summary.total_frames_written,
        total_samples_written: consumer_summary.total_samples_written,
        total_samples_popped: consumer_summary.total_samples_popped,
        total_samples_padded: consumer_summary.total_samples_padded,
        ring_watermark_max_samples: consumer_summary.ring_watermark_max,
        ring_write_samples: stats.ring_write_samples.load(Ordering::Relaxed),
        ring_write_events: stats.ring_write_events.load(Ordering::Relaxed),
        ring_write_blocked_events: stats.ring_write_blocked_events.load(Ordering::Relaxed),
        ring_discard_samples: stats.ring_discard_samples.load(Ordering::Relaxed),
        underrun_events: stats.underrun_events.load(Ordering::Relaxed),
        underrun_samples: stats.underrun_samples.load(Ordering::Relaxed),
        overrun_events: stats.overrun_events.load(Ordering::Relaxed),
        overrun_samples: stats.overrun_samples.load(Ordering::Relaxed),
        device_rebuilds: stats.device_rebuilds.load(Ordering::Relaxed),
        last_error: stats.last_error(),
        last_underrun_ring_fill_samples: stats
            .last_underrun_ring_fill_samples
            .load(Ordering::Relaxed),
        last_overrun_ring_fill_samples: stats
            .last_overrun_ring_fill_samples
            .load(Ordering::Relaxed),
        last_producer_blocked_ring_fill_samples: stats
            .last_producer_blocked_ring_fill_samples
            .load(Ordering::Relaxed),
    }
}

pub(super) fn log_smoke_summary(
    diag: &mut DiagWriter,
    summary: &SmokeTestSummary,
    consumer_summary: &ConsumerSummary,
) -> AudioResult<()> {
    diag.line(format!(
        "wasapi_shared: stats total_frames_written={} total_samples_written={} total_samples_popped={} total_samples_padded={} ring_watermark_max_samples={} ring_write_samples={} ring_write_events={} ring_write_blocked_events={} underrun_events={} underrun_samples={} device_rebuilds={} last_error={:?}",
        consumer_summary.total_frames_written,
        consumer_summary.total_samples_written,
        consumer_summary.total_samples_popped,
        consumer_summary.total_samples_padded,
        consumer_summary.ring_watermark_max,
        summary.ring_write_samples,
        summary.ring_write_events,
        summary.ring_write_blocked_events,
        summary.underrun_events,
        summary.underrun_samples,
        summary.device_rebuilds,
        summary.last_error,
    ))?;

    diag.line(format!("wasapi_shared: finished summary={summary:?}"))?;
    Ok(())
}
