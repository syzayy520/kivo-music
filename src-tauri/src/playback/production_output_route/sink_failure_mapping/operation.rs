#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(dead_code)]
pub(crate) enum ProductionOutputRouteSinkFailureOperation {
    SubmitFrame,
}

#[allow(dead_code)]
impl ProductionOutputRouteSinkFailureOperation {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SubmitFrame => "submit_frame",
        }
    }
}
