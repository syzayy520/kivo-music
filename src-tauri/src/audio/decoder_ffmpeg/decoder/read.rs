use crate::audio::errors::{AudioError, AudioResult};

use super::super::{ffi, resample, types::PcmChunkF32};
use super::FfmpegDecoder;

pub fn next_pcm_f32(dec: &mut FfmpegDecoder) -> AudioResult<Option<PcmChunkF32>> {
    loop {
        let r = unsafe { (dec.api.avcodec_receive_frame)(dec.codec_ctx, dec.frame) };
        if r == 0 {
            let chunk = resample::frame_to_pcm_f32(dec)?;
            unsafe { (dec.api.av_frame_unref)(dec.frame) };
            let ch = chunk.channels.max(1) as usize;
            let frames = (chunk.data.len() / ch) as u64;
            dec.frames_emitted = dec.frames_emitted.saturating_add(frames);
            return Ok(Some(chunk));
        }

        if r == ffi::AVERROR_EAGAIN {
            feed_one_packet(dec)?;
            continue;
        }

        if r == ffi::AVERROR_EOF {
            return Ok(None);
        }

        return Err(AudioError::FfmpegApiError {
            context: "avcodec_receive_frame",
            code: r,
            message: dec.api.error_string(r),
        });
    }
}

fn feed_one_packet(dec: &mut FfmpegDecoder) -> AudioResult<()> {
    loop {
        let r = unsafe { (dec.api.av_read_frame)(dec.fmt, dec.pkt) };
        if r == ffi::AVERROR_EOF {
            if !dec.sent_eof {
                let r2 = unsafe { (dec.api.avcodec_send_packet)(dec.codec_ctx, std::ptr::null()) };
                dec.sent_eof = true;
                if r2 < 0 {
                    return Err(AudioError::FfmpegApiError {
                        context: "avcodec_send_packet(NULL)",
                        code: r2,
                        message: dec.api.error_string(r2),
                    });
                }
            }
            return Ok(());
        }

        if r < 0 {
            return Err(AudioError::FfmpegApiError {
                context: "av_read_frame",
                code: r,
                message: dec.api.error_string(r),
            });
        }

        let pkt_stream = unsafe { (*dec.pkt).stream_index };
        if pkt_stream != dec.audio_stream_index {
            unsafe { (dec.api.av_packet_unref)(dec.pkt) };
            continue;
        }

        let r2 = unsafe { (dec.api.avcodec_send_packet)(dec.codec_ctx, dec.pkt) };
        unsafe { (dec.api.av_packet_unref)(dec.pkt) };
        if r2 == 0 || r2 == ffi::AVERROR_EAGAIN {
            return Ok(());
        }

        return Err(AudioError::FfmpegApiError {
            context: "avcodec_send_packet",
            code: r2,
            message: dec.api.error_string(r2),
        });
    }
}
