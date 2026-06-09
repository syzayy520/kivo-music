//! Runtime pump module.
//!
//! Pure-memory runtime pump bridging thread loop, driver, sink dispatch,
//! and event buffer into a single testable tick.

pub mod pump_context;
pub mod pump_error;
pub mod pump_outcome;
pub mod pump_step;

pub use pump_context::PumpContext;
pub use pump_error::PumpError;
pub use pump_outcome::PumpOutcome;
pub use pump_step::execute_pump_tick;
