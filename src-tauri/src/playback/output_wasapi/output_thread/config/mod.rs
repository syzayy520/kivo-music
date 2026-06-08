//! Output thread runtime configuration types.
//!
//! Pure data types representing runtime configuration for the output thread.
//! No behavior, no side effects.

pub mod buffer_config;
pub mod render_loop_config;
pub mod thread_config;

// Re-export primary types for convenience.
pub use buffer_config::BufferConfig;
pub use render_loop_config::RenderLoopConfig;
pub use thread_config::ThreadConfig;