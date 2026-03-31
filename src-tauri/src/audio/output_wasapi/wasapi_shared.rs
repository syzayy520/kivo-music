use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use wasapi::{initialize_mta, DeviceEnumerator, Direction, StreamMode};

use crate::audio::{
    diag::DiagWriter,
    error::AudioResult,
    ring,
    smoke::{SmokeTestConfig, SmokeTestSummary},
};

use super::{
    wasapi_shared_consumer::{run_consumer_loop, ConsumerConfig},
    wasapi_shared_producer::{spawn_producer, ProducerConfig},
    wasapi_shared_summary::{build_smoke_summary, log_smoke_summary},
    WasapiSoakStats,
};

const DEFAULT_BUFFER_DURATION_HNS: i64 = 1_000_000; // 100ms
pub fn run_wasapi_shared_smoke(
    config: &SmokeTestConfig,
    diag: &mut DiagWriter,
) -> AudioResult<SmokeTestSummary> {
    let h = initialize_mta();
    if h.is_err() {
        diag.line(format!("wasapi_shared: initialize_mta failed: {h:?}"))?;
    }

    // MUST NOT run on a UI thread.

    let enumerator = DeviceEnumerator::new()?;
    let device = enumerator.get_default_device(&Direction::Render)?;

    let device_name = device
        .get_friendlyname()
        .unwrap_or_else(|_| "<unknown device>".to_string());
    let device_id = device
        .get_id()
        .unwrap_or_else(|_| "<unknown id>".to_string());

    diag.line(format!(
        "wasapi_shared: default_device name=\"{}\" id=\"{}\"",
        device_name, device_id
    ))?;

    let mut audio_client = device.get_iaudioclient()?;
    let mix_format = audio_client.get_mixformat()?;

    let sample_rate = mix_format.get_samplespersec() as usize;
    let channels = mix_format.get_nchannels() as usize;
    let bits_per_sample = mix_format.get_bitspersample() as usize;
    let valid_bits = mix_format.get_validbitspersample() as usize;
    let block_align = mix_format.get_blockalign() as usize;
    let sample_type = mix_format.get_subformat()?;

    diag.line(format!(
        "wasapi_shared: mix_format sample_rate={} channels={} sample_type={:?} bits_per_sample={} valid_bits={} block_align={}",
        sample_rate, channels, sample_type, bits_per_sample, valid_bits, block_align
    ))?;

    diag.line(
        "wasapi_shared: strategy underrun=pad_silence + ramp_in overrun=producer_backpressure(no_drop)",
    )?;

    // Choose a fairly conservative buffer to keep underruns controllable.
    let buffer_duration_hns = DEFAULT_BUFFER_DURATION_HNS;
    let mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns,
    };

    audio_client.initialize_client(&mix_format, &Direction::Render, &mode)?;
    let handle = audio_client.set_get_eventhandle()?;
    let render_client = audio_client.get_audiorenderclient()?;

    let buffer_frames = audio_client.get_buffer_size()? as usize;
    diag.line(format!("wasapi_shared: buffer_frames={}", buffer_frames))?;

    // Ring buffer capacity: `ring_buffer_secs` seconds of interleaved f32 samples.
    let secs = config.ring_buffer_secs.max(0.2);
    let ring_capacity_samples = ((sample_rate as f32) * (channels as f32) * secs).ceil() as usize;
    let (prod, mut cons) = ring::make_f32_spsc(ring_capacity_samples);

    let stats = Arc::new(WasapiSoakStats::default());

    let stop = crossbeam_channel::bounded::<()>(1);
    let stop_tx = stop.0;
    let stop_rx = stop.1;

    let producer_join = spawn_producer(
        ProducerConfig {
            mode: config.mode,
            frequency_hz: config.frequency_hz,
            amplitude: config.amplitude,
            sample_rate,
            channels,
            ring_capacity_samples,
            stats: stats.clone(),
            stop_rx,
        },
        prod,
    );

    // Prime the endpoint buffer with silence.
    if buffer_frames > 0 {
        let silence_bytes = vec![0u8; buffer_frames * block_align];
        render_client.write_to_device(buffer_frames, &silence_bytes, None)?;
    }

    audio_client.start_stream()?;

    let started = Instant::now();
    let deadline = started + Duration::from_secs(config.duration_secs as u64);

    let consumer_summary = run_consumer_loop(
        &mut cons,
        diag,
        ConsumerConfig {
            deadline,
            started,
            log_every: Duration::from_millis(500),
            buffer_frames,
            channels,
            ring_capacity_samples,
            bits_per_sample,
            valid_bits,
            sample_type,
            stats: stats.clone(),
        },
        || {
            let _ = handle.wait_for_event(250);
        },
        || {
            audio_client
                .get_available_space_in_frames()
                .map(|v| v as usize)
                .map_err(|e| {
                    crate::audio::errors::AudioError::unsupported(format!(
                        "wasapi: get available frames failed: {e}"
                    ))
                })
        },
        |available_frames, packed| {
            render_client
                .write_to_device(available_frames, packed, None)
                .map_err(|e| {
                    crate::audio::errors::AudioError::unsupported(format!(
                        "wasapi: render write failed: {e}"
                    ))
                })
        },
    )?;

    // Stop threads / audio.
    let _ = stop_tx.send(());
    let _ = producer_join.join();

    audio_client.stop_stream().ok();

    let mut summary = build_smoke_summary(
        config.mode,
        diag,
        device_name,
        device_id,
        sample_rate,
        channels,
        buffer_frames,
        ring_capacity_samples,
        bits_per_sample,
        sample_type,
        &consumer_summary,
        &stats,
    );
    summary.duration_secs = f64::from(config.duration_secs);
    log_smoke_summary(diag, &summary, &consumer_summary)?;
    Ok(summary)
}
