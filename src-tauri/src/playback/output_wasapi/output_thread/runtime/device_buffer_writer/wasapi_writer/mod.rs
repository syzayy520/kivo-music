//! WASAPI device buffer writer subfamily.
//!
//! Type scaffold for real WASAPI device buffer writer.
//! No real WASAPI calls, no COM objects, no audio data.

pub mod config;
pub mod state;
pub mod writer;

pub use config::WasapiDeviceBufferWriterConfig;
pub use state::WasapiDeviceBufferWriterState;
pub use writer::WasapiDeviceBufferWriter;
