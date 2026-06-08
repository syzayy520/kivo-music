use super::operation::OutputSubmitOperation;

// Contract-only until a route adapter or drain observation ticket consumes classifications.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct OutputSubmitErrorClassification {
    operation: OutputSubmitOperation,
    message: String,
}

// Contract-only until a route adapter or drain observation ticket adds a live caller.
#[allow(dead_code)]
impl OutputSubmitErrorClassification {
    pub(crate) fn new(operation: OutputSubmitOperation, message: String) -> Self {
        Self { operation, message }
    }

    pub(crate) fn operation(&self) -> OutputSubmitOperation {
        self.operation
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}
