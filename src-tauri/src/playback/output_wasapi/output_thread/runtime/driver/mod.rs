//! Output thread runtime driver types.
//!
//! Pure data types representing the driver shell for the output thread.
//! No actual driving logic — only data definitions for driver steps and results.

pub mod driver_result;
pub mod driver_step;
pub mod thread_driver;

// Re-export primary types for convenience.
pub use driver_result::DriverResult;
pub use driver_step::DriverStep;
pub use thread_driver::ThreadDriver;
