/// Very small linear resampler for interleaved f32 PCM.
///
/// This is NOT audiophile-grade; it is sufficient for shell smoke / E2E bring-up.
#[derive(Debug, Clone)]
pub struct LinearResampler {
    src_rate: u32,
    dst_rate: u32,
    channels: usize,
    pos: f64,
    step: f64,
}

impl LinearResampler {
    pub fn new(src_rate: u32, dst_rate: u32, channels: usize) -> Self {
        let step = (src_rate as f64) / (dst_rate as f64);
        Self {
            src_rate,
            dst_rate,
            channels: channels.max(1),
            pos: 0.0,
            step,
        }
    }

    pub fn is_identity(&self) -> bool {
        self.src_rate == self.dst_rate
    }

    /// Resample `input` into `out`.
    ///
    /// - `input` is interleaved with `channels`.
    /// - `out` is replaced.
    pub fn resample_interleaved(&mut self, input: &[f32], out: &mut Vec<f32>) {
        out.clear();
        if self.is_identity() {
            out.extend_from_slice(input);
            return;
        }

        let in_frames = input.len() / self.channels;
        if in_frames == 0 {
            return;
        }

        // Conservative output frame count estimate for current chunk.
        // We generate until we would require a sample beyond the last input frame.
        let mut out_frames: usize = 0;
        let mut p = self.pos;
        while (p + 1.0) < (in_frames as f64) {
            out_frames = out_frames.saturating_add(1);
            p += self.step;
        }

        out.reserve(out_frames.saturating_mul(self.channels));

        let mut pos = self.pos;
        for _ in 0..out_frames {
            let i0 = pos.floor() as usize;
            let frac = (pos - (i0 as f64)) as f32;
            let i1 = (i0 + 1).min(in_frames - 1);

            for ch in 0..self.channels {
                let s0 = input[i0 * self.channels + ch];
                let s1 = input[i1 * self.channels + ch];
                out.push(s0 + (s1 - s0) * frac);
            }

            pos += self.step;
        }

        self.pos = pos - (in_frames as f64);
        if self.pos < 0.0 {
            self.pos = 0.0;
        }
    }
}
