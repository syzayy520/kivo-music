use crate::playback::audio_route_pipeline_tap::{
    AudioRoutePipelineTapErrorKind, AudioRoutePipelineTapReport,
};
use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::playback) struct TapDiagnosticReportSnapshot {
    pub enabled: bool,
    pub current: Option<TapDiagnosticCompactReport>,
    pub last_detached: Option<TapDiagnosticCompactReport>,
    pub current_stream: Option<TapDiagnosticStreamSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::playback) struct TapDiagnosticCompactReport {
    pub initialized: bool,
    pub closed: bool,
    pub input_count: u64,
    pub total_accepted_frames: u64,
    pub total_rejected_frames: u64,
    pub backpressure_count: u64,
    pub partial_write_count: u64,
    pub source_closed_seen: bool,
    pub pending_frames: u32,
    pub capacity_frames: u32,
    pub last_error: Option<AudioRoutePipelineTapErrorKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::playback) struct TapDiagnosticStreamSnapshot {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub sample_format: TapDiagnosticSampleFormatSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::playback) enum TapDiagnosticSampleFormatSnapshot {
    Float32,
    Signed16,
    Signed24,
    Signed32,
}

impl TapDiagnosticReportSnapshot {
    pub(in crate::playback) fn disabled() -> Self {
        Self {
            enabled: false,
            current: None,
            last_detached: None,
            current_stream: None,
        }
    }

    pub(super) fn from_native_engine(engine: &KivoNativeEngine) -> Self {
        if !engine.tap_diagnostic_is_enabled() {
            return Self::disabled();
        }

        Self {
            enabled: true,
            current: engine
                .tap_diagnostic_current_report()
                .as_ref()
                .map(TapDiagnosticCompactReport::from_report),
            last_detached: engine
                .tap_diagnostic_last_detached_report()
                .as_ref()
                .map(TapDiagnosticCompactReport::from_report),
            current_stream: engine
                .tap_diagnostic_current_stream()
                .map(TapDiagnosticStreamSnapshot::from_stream),
        }
    }
}

impl TapDiagnosticCompactReport {
    fn from_report(report: &AudioRoutePipelineTapReport) -> Self {
        Self {
            initialized: report.initialized,
            closed: report.closed,
            input_count: report.input_count,
            total_accepted_frames: report.total_accepted_frames,
            total_rejected_frames: report.total_rejected_frames,
            backpressure_count: report.backpressure_count,
            partial_write_count: report.partial_write_count,
            source_closed_seen: report.source_closed_seen,
            pending_frames: report.pending_frames,
            capacity_frames: report.capacity_frames,
            last_error: report.last_error,
        }
    }
}

impl TapDiagnosticStreamSnapshot {
    fn from_stream(stream: &AudioStreamInfo) -> Self {
        Self {
            sample_rate_hz: stream.sample_rate_hz,
            channels: stream.channels,
            sample_format: TapDiagnosticSampleFormatSnapshot::from_format(&stream.sample_format),
        }
    }
}

impl TapDiagnosticSampleFormatSnapshot {
    fn from_format(format: &AudioSampleFormat) -> Self {
        match format {
            AudioSampleFormat::Float32 => Self::Float32,
            AudioSampleFormat::Signed16 => Self::Signed16,
            AudioSampleFormat::Signed24 => Self::Signed24,
            AudioSampleFormat::Signed32 => Self::Signed32,
        }
    }
}
