// report_builders.rs
//
// Facade for WasapiRenderClientSmokeReport builder methods.
//
// Actual builders are split into focused sibling modules:
// - report_defaults: base constructors with safe defaults
// - report_skipped_builders: skipped report constructors
// - report_failure_builders: failure report constructors
// - report_success_builders: success report constructors
//
// This file exists to preserve the module path for any code that
// imports from `report_builders`. All builder impl blocks live
// in the sibling modules.
