use ringbuf::{traits::*, HeapCons, HeapProd, HeapRb};

pub type F32Producer = HeapProd<f32>;
pub type F32Consumer = HeapCons<f32>;

/// Creates a SPSC ring buffer for interleaved f32 samples.
///
/// We store samples in interleaved layout: [L0, R0, L1, R1, ...].
pub fn make_f32_spsc(capacity_samples: usize) -> (F32Producer, F32Consumer) {
    let rb = HeapRb::<f32>::new(capacity_samples);
    rb.split()
}
