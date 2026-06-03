// reset_boundary/report_builders.rs
//
// Facade for WasapiResetBoundarySmokeReport builder methods.
//
// Actual builders are split into focused sibling modules:
// - report_defaults: base constructors with safe defaults
// - report_skipped_builders: skipped report constructors
// - report_failure_prereq_builders: prerequisite failure constructors
// - report_failure_buffer_builders: buffer failure constructors
// - report_failure_reset_builders: reset failure constructors
// - report_success_builders: success report constructors
//
// This file exists to preserve the module path for any code that
// imports from `report_builders`. All builder impl blocks live
// in the sibling modules.
