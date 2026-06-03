// report_failure_builders.rs
//
// Facade for failure report builder methods.
//
// The actual builder implementations are split into specialized sibling
// modules for maintainability (single-file ≤220 lines governance):
//
// - report_failure_prereq_builders: Endpoint activation, mix format,
//   initialization, service acquisition, and GetBufferSize failures.
// - report_failure_buffer_builders: Buffer size zero, GetBuffer,
//   ReleaseBuffer, and IAudioClient::Start failures.
// - report_failure_padding_builders: IAudioClient::GetCurrentPadding
//   and IAudioClient::Stop failures.
//
// Each sibling module implements methods on WasapiPaddingQuerySmokeReport
// via the `impl WasapiPaddingQuerySmokeReport` pattern.
// This file is intentionally a documentation-only facade.
