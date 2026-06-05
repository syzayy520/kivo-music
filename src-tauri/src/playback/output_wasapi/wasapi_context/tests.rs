//! Tests for WasapiContext.
//!
//! These tests verify the cross-platform behavior of WasapiContext
//! without requiring real audio hardware.

use super::WasapiContext;

#[test]
fn new_context_is_not_open() {
    let ctx = WasapiContext::new();
    assert!(!ctx.is_open());
    assert!(!ctx.has_render_client());
}

#[test]
fn close_before_open_is_noop() {
    let mut ctx = WasapiContext::new();
    ctx.close();
    assert!(!ctx.is_open());
    assert!(!ctx.has_render_client());
}

#[test]
fn default_is_not_open() {
    let ctx = WasapiContext::default();
    assert!(!ctx.is_open());
    assert!(!ctx.has_render_client());
}

#[test]
fn debug_format_shows_state() {
    let ctx = WasapiContext::new();
    let debug = format!("{ctx:?}");
    assert!(debug.contains("is_open: false"));
    assert!(debug.contains("has_render_client: false"));
}

#[test]
fn close_twice_is_idempotent() {
    let mut ctx = WasapiContext::new();
    ctx.close();
    ctx.close();
    assert!(!ctx.is_open());
    assert!(!ctx.has_render_client());
}

#[test]
fn open_returns_error_on_non_windows() {
    let mut ctx = WasapiContext::new();
    let result = ctx.open();

    #[cfg(not(target_os = "windows"))]
    {
        assert!(result.is_err());
        assert!(!ctx.is_open());
        assert!(!ctx.has_render_client());
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, open may succeed or fail depending on device availability
        let _ = result;
    }
}

#[test]
fn repeated_open_clears_old_context() {
    let mut ctx = WasapiContext::new();

    // First open attempt
    let _ = ctx.open();

    // Second open attempt should clear old context first
    let _ = ctx.open();

    // On non-Windows, both should fail
    #[cfg(not(target_os = "windows"))]
    {
        assert!(!ctx.is_open());
        assert!(!ctx.has_render_client());
    }
}

#[test]
fn close_after_failed_open_is_noop() {
    let mut ctx = WasapiContext::new();
    let _ = ctx.open(); // Will fail on non-Windows
    ctx.close();
    assert!(!ctx.is_open());
    assert!(!ctx.has_render_client());
}
