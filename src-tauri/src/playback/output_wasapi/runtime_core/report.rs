use super::super::output_thread_core::errors::OutputThreadErrorSummary;
use super::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::super::output_thread_core::state::{OutputThreadReport, OutputThreadStats};

/// Summary of an output thread runtime report.
///
/// Pure report wrapper — no real receiver, no real thread.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct OutputThreadRuntimeReportSummary {
    /// Runtime identifier.
    pub id: OutputThreadRuntimeId,
    /// Runtime generation.
    pub generation: OutputThreadRuntimeGeneration,
    /// Final statistics snapshot.
    pub stats: OutputThreadStats,
    /// Error summary if the thread exited due to an error.
    pub error: Option<OutputThreadErrorSummary>,
    /// Whether the thread exited due to a panic.
    pub panicked: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeReportSummary {
    /// Create a new report summary with explicit values.
    pub(crate) fn new(
        id: OutputThreadRuntimeId,
        generation: OutputThreadRuntimeGeneration,
        stats: OutputThreadStats,
        error: Option<OutputThreadErrorSummary>,
        panicked: bool,
    ) -> Self {
        Self {
            id,
            generation,
            stats,
            error,
            panicked,
        }
    }

    /// Create a report summary from a thread report.
    pub(crate) fn from_thread_report(
        id: OutputThreadRuntimeId,
        generation: OutputThreadRuntimeGeneration,
        report: OutputThreadReport,
    ) -> Self {
        Self {
            id,
            generation,
            stats: report.stats,
            error: report.error.map(|msg| {
                OutputThreadErrorSummary::new(
                    super::super::output_thread_core::errors::OutputThreadErrorKind::ThreadPanic,
                    msg,
                )
            }),
            panicked: report.panicked,
        }
    }

    /// Whether the report contains an error.
    pub(crate) fn has_error(&self) -> bool {
        self.error.is_some() || self.panicked
    }

    /// Whether the thread panicked.
    pub(crate) fn panicked(&self) -> bool {
        self.panicked
    }

    /// Number of frames rendered to the output device.
    pub(crate) fn rendered_frames(&self) -> u64 {
        self.stats.rendered_frames
    }

    /// Number of silence frames filled.
    pub(crate) fn silence_frames(&self) -> u64 {
        self.stats.silence_filled_frames
    }
}
