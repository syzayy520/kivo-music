use std::{
    io::Write,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

use ringbuf::traits::Producer;

use crate::audio::{
    decoder_ffmpeg::{header_probe, FfmpegDecoder, FfmpegLoadOptions},
    pipeline::{channel_map::map_channels_interleaved, linear_resampler::LinearResampler},
};

#[derive(Debug, Clone)]
pub(super) struct ProducerMeta {
    pub(super) ffmpeg_version: String,
    pub(super) input_sample_rate_hz: u32,
    pub(super) input_channels: u16,
}

#[derive(Debug, Clone)]
pub(super) enum ProducerMsg {
    Meta(ProducerMeta),
    Fatal(String),
}

pub(super) struct ProducerConfig {
    pub(super) input_path: PathBuf,
    pub(super) start: f64,
    pub(super) max: f64,
    pub(super) load_opt: FfmpegLoadOptions,
    pub(super) device_sr: u32,
    pub(super) device_ch: u16,
    pub(super) diag_log_path: Option<PathBuf>,
    pub(super) done: Arc<AtomicBool>,
    pub(super) meta_tx: mpsc::SyncSender<ProducerMsg>,
}

pub(super) fn spawn_producer(
    cfg: ProducerConfig,
    mut prod: impl Producer<Item = f32> + Send + 'static,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut log = cfg
            .diag_log_path
            .as_ref()
            .and_then(|p| std::fs::File::create(p).ok())
            .map(std::io::BufWriter::new);

        let mut write_line = |s: &str| {
            if let Some(w) = log.as_mut() {
                let _ = writeln!(w, "{s}");
            }
        };

        write_line(&format!(
            "producer: input={} start={:.3} max={:.3} device_sr={} device_ch={}",
            cfg.input_path.display(),
            cfg.start,
            cfg.max,
            cfg.device_sr,
            cfg.device_ch
        ));

        let mut decoder = match FfmpegDecoder::open(&cfg.input_path, &cfg.load_opt) {
            Ok(d) => d,
            Err(e) => {
                let msg = format!("ffmpeg open failed: {e}");
                write_line(&format!("producer: {msg}"));
                let _ = cfg.meta_tx.send(ProducerMsg::Fatal(msg));
                cfg.done.store(true, Ordering::SeqCst);
                return;
            }
        };

        let ffmpeg_version = decoder.ffmpeg_version();
        if cfg.start > 0.0 {
            if let Err(e) = decoder.seek_seconds(cfg.start) {
                write_line(&format!("producer: seek failed: {e}"));
            }
        }

        let input_sr = header_probe::probe_sample_rate(&cfg.input_path).unwrap_or(cfg.device_sr);
        let mut input_ch: u16 = 0;
        let mut sent_meta = false;

        let mut tmp_mapped: Vec<f32> = Vec::new();
        let mut tmp_resampled: Vec<f32> = Vec::new();
        let mut resampler = LinearResampler::new(input_sr, cfg.device_sr, cfg.device_ch as usize);

        let max_frames_total: u64 = (cfg.max * (cfg.device_sr as f64)).round().max(0.0) as u64;
        let mut frames_sent: u64 = 0;

        loop {
            if frames_sent >= max_frames_total {
                break;
            }

            let chunk = match decoder.next_pcm_f32() {
                Ok(c) => c,
                Err(e) => {
                    write_line(&format!("producer: decoder error: {e}"));
                    break;
                }
            };
            let Some(chunk) = chunk else { break };

            if input_ch == 0 {
                input_ch = chunk.channels;
            }

            if !sent_meta {
                sent_meta = true;
                let _ = cfg.meta_tx.send(ProducerMsg::Meta(ProducerMeta {
                    ffmpeg_version: ffmpeg_version.clone(),
                    input_sample_rate_hz: input_sr,
                    input_channels: input_ch,
                }));
                write_line(&format!(
                    "producer: meta ffmpeg_version={} input_sr={} input_ch={}",
                    ffmpeg_version, input_sr, input_ch
                ));
            }

            let src_ch = (chunk.channels as usize).max(1);
            let dst_ch = (cfg.device_ch as usize).max(1);

            map_channels_interleaved(&chunk.data, src_ch, dst_ch, &mut tmp_mapped);
            resampler.resample_interleaved(&tmp_mapped, &mut tmp_resampled);

            let remain_frames = max_frames_total.saturating_sub(frames_sent);
            let remain_samples = (remain_frames as usize).saturating_mul(dst_ch);
            if tmp_resampled.len() > remain_samples {
                tmp_resampled.truncate(remain_samples);
            }

            let chunk_samples = tmp_resampled.len();
            if chunk_samples == 0 {
                continue;
            }

            let mut offset = 0usize;
            while offset < chunk_samples {
                let vacant = prod.vacant_len();
                if vacant == 0 {
                    thread::sleep(Duration::from_millis(2));
                    continue;
                }
                let n = (chunk_samples - offset).min(vacant);
                let wrote = prod.push_slice(&tmp_resampled[offset..offset + n]);
                offset += wrote;
            }

            frames_sent = frames_sent.saturating_add((chunk_samples / dst_ch) as u64);
        }

        write_line(&format!("producer: done frames_sent={frames_sent}"));
        cfg.done.store(true, Ordering::SeqCst);
    })
}
