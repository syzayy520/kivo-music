//! Fake render client boundary implementation.
//!
//! Pure-memory RenderClientBoundary implementation for testing.
//! Simulates buffer acquire/release/query operations without real WASAPI calls.
//! No COM objects, no Windows API calls, no audio data processing.

mod buffer_simulation;
mod client;
mod trait_impl;

pub use client::FakeRenderClientBoundary;

#[cfg(test)]
mod tests;
