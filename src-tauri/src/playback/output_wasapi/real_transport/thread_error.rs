//! Stable error types for real output thread skeleton.
//!
//! Covers spawn, shutdown, and join failures without
//! referencing COM, WASAPI, or audio primitives.

use super::super::errors::WasapiOpenError;
use super::super::sink_drain::WasapiOutputThreadOwnedStateError;
use super::channel::OutputThreadRealTransportSendError;

/// Errors from real output thread spawn, shutdown, and join.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RealOutputThreadSkeletonError {
    /// Owned-state validation failed inside the thread.
    OwnedStateValidation(WasapiOutputThreadOwnedStateError),
    /// Sending shutdown command failed.
    SendShutdown(OutputThreadRealTransportSendError),
    /// Thread panicked during join.
    JoinPanic,
    /// Thread exited without returning a report.
    WorkerDidNotReport,
    /// WasapiContext::open() failed inside the thread.
    WasapiContextOpen(WasapiOpenError),
}
