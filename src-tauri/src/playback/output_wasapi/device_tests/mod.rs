// device_tests/mod.rs
//
// Tests for the WASAPI endpoint smoke boundary.
// Split from legacy device_tests.rs for single-responsibility governance.

mod env_tests;
mod regression_tests;
mod smoke_ignored_tests;

// report_tests not needed: device endpoint smoke has no dedicated report struct
// to validate independently. Report field assertions live in env_tests and
// smoke_ignored_tests where they are contextually relevant.
