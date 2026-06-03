// report_failure_builders.rs
//
// Facade for failure report builder methods.
//
// The actual builder implementations are split into specialized sibling
// modules for maintainability (single-file ≤220 lines governance):
//
// - report_failure_prereq_builders: Endpoint activation, mix format,
//   initialization, and service acquisition failures.
// - report_failure_buffer_builders: Buffer size validation, GetBuffer,
//   and ReleaseBuffer failures.
// - report_failure_start_stop_builders: IAudioClient::Start and Stop failures.
//
// Each sibling module implements methods on WasapiStartStopSmokeReport
// via the `impl WasapiStartStopSmokeReport` pattern.
// This file is intentionally a documentation-only facade.
