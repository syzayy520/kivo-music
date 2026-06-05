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
    /// IAudioClient::Start was requested without opening WasapiContext.
    AudioClientStartRequiresOpenContext,
    /// IAudioClient::Start failed inside the thread.
    AudioClientStartFailed(String),
    /// IAudioClient::Stop failed inside the thread.
    AudioClientStopFailed(String),
    /// Render silence once was requested without opening WasapiContext.
    RenderSilenceOnceRequiresOpenContext,
    /// Render silence once was requested with zero frames.
    RenderSilenceOnceInvalidFrameCount,
    /// Render silence once failed inside the thread.
    RenderSilenceOnceFailed(String),
    /// Render silence loop was requested without opening WasapiContext.
    RenderSilenceLoopRequiresOpenContext,
    /// Render silence loop was requested before audio client start succeeded.
    RenderSilenceLoopRequiresStartedClient,
    /// Render silence loop was requested with zero iterations.
    RenderSilenceLoopInvalidIterationCount,
    /// Render silence loop was requested with zero frames per write.
    RenderSilenceLoopInvalidFrameCount,
    /// Render silence loop iteration count exceeded the ticket bound.
    RenderSilenceLoopIterationCountTooLarge,
    /// Render silence loop frames per write exceeded the ticket bound.
    RenderSilenceLoopFrameCountTooLarge,
    /// Render silence loop failed inside the thread.
    RenderSilenceLoopFailed(String),
    /// Padding-aware render loop was requested without opening WasapiContext.
    RenderPaddingLoopRequiresOpenContext,
    /// Padding-aware render loop was requested before audio client start succeeded.
    RenderPaddingLoopRequiresStartedClient,
    /// Padding-aware render loop was requested with zero iterations.
    RenderPaddingLoopInvalidIterationCount,
    /// Padding-aware render loop was requested with zero frames per write.
    RenderPaddingLoopInvalidFrameCount,
    /// Padding-aware render loop iteration count exceeded the ticket bound.
    RenderPaddingLoopIterationCountTooLarge,
    /// Padding-aware render loop frames per write exceeded the ticket bound.
    RenderPaddingLoopFrameCountTooLarge,
    /// Padding-aware render loop failed while querying padding state.
    RenderPaddingLoopPaddingStateFailed(String),
    /// Padding-aware render loop failed while writing silence.
    RenderPaddingLoopWriteFailed(String),
}
