#[derive(Debug, Clone)]
pub struct PcmChunkF32 {
    /// Interleaved samples.
    pub data: Vec<f32>,
    pub channels: u16,
}
