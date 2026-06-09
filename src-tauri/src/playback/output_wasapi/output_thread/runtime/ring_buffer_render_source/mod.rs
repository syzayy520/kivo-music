//! Ring buffer render source types.
//!
//! Pure data types for a ring buffer render source implementation.
//! No actual audio data, no WASAPI, no IO, no threads.
//! Device-agnostic boundary for future real RingBuffer consumer.

pub mod buffer_cursor;
pub mod buffer_packet;
pub mod buffer_state;
pub mod read_error;
pub mod read_result;

// Re-export primary types for convenience.
pub use buffer_cursor::BufferCursor;
pub use buffer_packet::BufferPacket;
pub use buffer_state::BufferState;
pub use read_error::RingBufferReadError;
pub use read_result::RingBufferReadResult;
