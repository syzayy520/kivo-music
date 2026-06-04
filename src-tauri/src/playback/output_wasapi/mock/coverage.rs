/// Coverage tracker for mock scenarios.
///
/// Pure data — tracks which scenarios have been run.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadMockCoverage {
    pub normal_audio: bool,
    pub empty_running: bool,
    pub no_capacity: bool,
    pub shutdown_requested: bool,
    pub paused_empty: bool,
    pub paused_with_frames: bool,
    pub flush_empty: bool,
    pub closed_empty: bool,
    pub closed_with_remaining: bool,
    pub non_running: bool,
}

/// Total number of expected scenarios.
const EXPECTED_COUNT: usize = 10;

#[allow(dead_code)]
impl OutputThreadMockCoverage {
    /// Build coverage from a list of scenario names.
    pub(crate) fn from_names(names: &[&'static str]) -> Self {
        let mut c = Self::default();
        for name in names {
            match *name {
                "normal_audio" => c.normal_audio = true,
                "empty_running" => c.empty_running = true,
                "no_capacity" => c.no_capacity = true,
                "shutdown_requested" => c.shutdown_requested = true,
                "paused_empty" => c.paused_empty = true,
                "paused_with_frames" => c.paused_with_frames = true,
                "flush_empty" => c.flush_empty = true,
                "closed_empty" => c.closed_empty = true,
                "closed_with_remaining" => c.closed_with_remaining = true,
                "non_running" => c.non_running = true,
                _ => {}
            }
        }
        c
    }

    /// Whether all 10 scenarios are covered.
    pub(crate) fn is_complete(&self) -> bool {
        self.covered_count() == EXPECTED_COUNT
    }

    /// Number of scenarios covered.
    pub(crate) fn covered_count(&self) -> usize {
        let mut count = 0;
        if self.normal_audio {
            count += 1;
        }
        if self.empty_running {
            count += 1;
        }
        if self.no_capacity {
            count += 1;
        }
        if self.shutdown_requested {
            count += 1;
        }
        if self.paused_empty {
            count += 1;
        }
        if self.paused_with_frames {
            count += 1;
        }
        if self.flush_empty {
            count += 1;
        }
        if self.closed_empty {
            count += 1;
        }
        if self.closed_with_remaining {
            count += 1;
        }
        if self.non_running {
            count += 1;
        }
        count
    }

    /// Expected number of scenarios (always 10).
    pub(crate) fn expected_count(&self) -> usize {
        EXPECTED_COUNT
    }

    /// Number of scenarios not yet covered.
    pub(crate) fn missing_count(&self) -> usize {
        EXPECTED_COUNT - self.covered_count()
    }
}
