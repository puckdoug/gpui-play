// Async task state management types.

/// State for the async demo, tracking task status and results.
pub struct AsyncDemo {
    status: TaskStatus,
    result: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Idle,
    Running,
    Complete,
    Cancelled,
}

impl AsyncDemo {
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Idle,
            result: None,
        }
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn result(&self) -> Option<&str> {
        self.result.as_deref()
    }

    /// Set status to Running.
    pub fn start(&mut self) {
        todo!("implement start")
    }

    /// Set status to Complete with a result.
    pub fn complete(&mut self, result: String) {
        todo!("implement complete")
    }

    /// Set status to Cancelled.
    pub fn cancel(&mut self) {
        todo!("implement cancel")
    }
}

/// Pure function: a "heavy computation" that can run on background thread.
/// Returns a result string after processing.
pub fn heavy_computation(input: i32) -> String {
    todo!("implement heavy_computation")
}
