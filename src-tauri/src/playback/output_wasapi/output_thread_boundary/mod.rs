// output_thread_boundary/mod.rs
//
// Module declarations for output thread boundary smoke.

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_buffer_builders;
pub mod report_failure_prereq_builders;
pub mod report_failure_reset_builders;
pub mod report_failure_thread_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;
pub mod thread_report;
pub mod thread_runner;

// Platform-specific modules
#[cfg(target_os = "windows")]
pub mod output_thread_steps;
#[cfg(target_os = "windows")]
pub mod output_thread_windows;

// Non-Windows stub
#[cfg(not(target_os = "windows"))]
pub mod output_thread_stub;

// Re-export the main entry point for convenience
#[cfg(target_os = "windows")]
pub use output_thread_windows::probe_output_thread_boundary;

#[cfg(not(target_os = "windows"))]
pub use output_thread_stub::probe_output_thread_boundary;
