#[cfg(test)]
mod tests;

pub mod error;
pub mod report;
pub mod source;
pub mod source_to_ring_buffer;
pub mod types;

pub use error::SourceToRingBufferBridgeError;
pub use report::SourceToRingBufferBridgeReport;
pub use source::{from_decoded_frame, from_output_frame, from_parts};
pub use source_to_ring_buffer::write_pcm_source_chunk_to_ring_buffer;
pub use types::{PcmSourceChunk, PcmSourceFormat};
