#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteSinkFailure {
    operation: String,
    message: String,
}

impl ProductionOutputRouteSinkFailure {
    pub(crate) fn new(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            message: message.into(),
        }
    }

    pub(crate) fn operation(&self) -> &str {
        &self.operation
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}
