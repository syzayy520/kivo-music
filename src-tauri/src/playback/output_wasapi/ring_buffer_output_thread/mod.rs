// ring_buffer_output_thread/mod.rs
//
// Module declarations for ring buffer output thread smoke.

pub mod env;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod thread_report;

// Platform-specific modules
#[cfg(target_os = "windows")]
pub mod format_fields;
#[cfg(target_os = "windows")]
pub mod guards;
#[cfg(target_os = "windows")]
pub mod output_thread_flow;
#[cfg(target_os = "windows")]
pub mod output_thread_outcome;
#[cfg(target_os = "windows")]
pub mod output_thread_steps;
#[cfg(target_os = "windows")]
pub mod output_thread_windows;
#[cfg(target_os = "windows")]
pub mod thread_runner;

// Non-Windows stub
#[cfg(not(target_os = "windows"))]
pub mod output_thread_stub;

// Re-export the main entry point for convenience
#[cfg(target_os = "windows")]
pub use output_thread_windows::probe_ring_buffer_output_thread_smoke;

#[cfg(not(target_os = "windows"))]
pub use output_thread_stub::probe_ring_buffer_output_thread_smoke;

pub use report::WasapiRingBufferOutputThreadSmokeReport;
