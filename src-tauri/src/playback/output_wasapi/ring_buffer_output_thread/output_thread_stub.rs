use super::env;
use super::report::WasapiRingBufferOutputThreadSmokeReport;

/// Entry point for ring buffer output thread smoke boundary.
/// Current scaffold: no WASAPI flow, no thread spawn, no ring buffer creation.
pub fn probe_ring_buffer_output_thread_smoke() -> WasapiRingBufferOutputThreadSmokeReport {
    if !env::is_opt_in_enabled() {
        return WasapiRingBufferOutputThreadSmokeReport::skipped_env_missing();
    }
    // Scaffold: no Windows WASAPI flow yet (P0-051B)
    // Return scaffold-ready report for both Windows and non-Windows
    WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report()
}
