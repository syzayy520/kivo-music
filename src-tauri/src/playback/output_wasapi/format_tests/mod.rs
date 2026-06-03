// format_tests/mod.rs
//
// Tests for the WASAPI mix format smoke boundary.
//
// This module contains tests split by dimension:
// - env_tests: opt-in environment variable behavior
// - report_tests: report field defaults and constraints
// - pointer_tests: format pointer and field extraction
// - regression_tests: existing behavior preserved
// - smoke_ignored_tests: real COM smoke test (ignored by default)

mod env_tests;
mod pointer_tests;
mod regression_tests;
mod report_tests;
mod smoke_ignored_tests;
