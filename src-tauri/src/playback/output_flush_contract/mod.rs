mod ack;
mod decision;
mod failure;
mod ordering;
mod request;
mod target;

// Explicit type re-exports — no wildcard, no child module path exposure.
pub use ack::{
    OutputFlushAck, OutputFlushCompletedDetails, OutputFlushFailedReason,
    OutputFlushRejectedReason, OutputFlushStaleGenerationDetails,
};
pub use decision::{
    OutputFlushDecision, OutputFlushProceed, OutputFlushRejectReason, OutputFlushWasiBarrier,
};
pub use failure::OutputFlushFailure;
pub use ordering::FlushOrdering;
pub use request::OutputFlushRequest;
pub use target::FlushTarget;

#[cfg(test)]
mod tests;
