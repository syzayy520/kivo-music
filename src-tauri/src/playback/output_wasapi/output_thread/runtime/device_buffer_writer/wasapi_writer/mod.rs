//! WASAPI device buffer writer subfamily.
//!
//! Type scaffold for real WASAPI device buffer writer.
//! No real WASAPI calls, no COM objects, no audio data.

pub mod config;
pub mod packet_write;
pub mod request_validation;
pub mod runtime_mode;
pub mod state;
pub mod writer;

pub use config::WasapiDeviceBufferWriterConfig;
pub use runtime_mode::{Readiness, RuntimeKind, RuntimeMode};
pub use state::{BufferLifecycle, WasapiDeviceBufferWriterState};
pub use writer::WasapiDeviceBufferWriter;
