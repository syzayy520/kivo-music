#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// Contract-only until a route adapter or drain observation ticket adds a live caller.
#[allow(dead_code)]
pub(crate) enum OutputSubmitOperation {
    SubmitFrame,
}

// Contract-only until a route adapter or drain observation ticket projects this label.
#[allow(dead_code)]
impl OutputSubmitOperation {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SubmitFrame => "submit_frame",
        }
    }
}
