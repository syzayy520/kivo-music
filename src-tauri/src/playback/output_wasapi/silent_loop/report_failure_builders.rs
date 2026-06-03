// silent_loop/report_failure_builders.rs
//
// Facade for failure report builder methods.
//
// The actual builder implementations are split into specialized sibling
// modules for maintainability (single-file ≤220 lines governance):
//
// - report_failure_prereq_builders: Endpoint activation, mix format,
//   initialization, service acquisition, and GetBufferSize failures.
// - report_failure_buffer_builders: Buffer size zero, prefill GetBuffer,
//   prefill ReleaseBuffer, and IAudioClient::Start failures.
// - report_failure_loop_builders: Loop GetCurrentPadding failures,
//   loop GetBuffer failures, loop ReleaseBuffer failures, and Stop failures.
//
// Each sibling module implements methods on WasapiSilentLoopSmokeReport
// via the `impl WasapiSilentLoopSmokeReport` pattern.
// This file is intentionally a documentation-only facade.
