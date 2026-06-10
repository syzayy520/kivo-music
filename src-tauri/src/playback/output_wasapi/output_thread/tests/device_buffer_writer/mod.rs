//! Device buffer writer tests.
//!
//! Tests are organized into semantic subfamilies:
//! - `contract/` — type-level and trait contract tests
//! - `cursor/` — WriterCursor and WriterState property tests
//! - `lifecycle/` — BufferLifecycle state machine tests
//! - `validation/` — invariant guards and request validation tests
//! - `telemetry/` — health, pressure, streak, and snapshot diff tests
//! - `runtime_placeholder/` — simulated buffer behavior tests
//! - `fake_writer/` — FakeDeviceBufferWriter test double tests
//! - `sink_integration/` — helper function integration tests

pub mod contract;
pub mod cursor;
pub mod fake_writer;
pub mod lifecycle;
pub mod runtime_placeholder;
pub mod sink_integration;
pub mod telemetry;
pub mod validation;
