//! `StageCompletedBuilder` — fluent constructor for [`StageCompleted`](super::StageCompleted).

use crate::api::event::types::stage::StageCompleted;

/// Fluent builder for [`StageCompleted`] events.
///
/// `occurred_at` is stamped to `SystemTime::now()` at [`build`](Self::build) time,
/// matching the behaviour of [`StageCompleted::new`].
#[derive(Default)]
pub struct StageCompletedBuilder {
    stage: String,
    handler_id: String,
    duration_ms: u64,
}

impl StageCompletedBuilder {
    /// Create a new builder with empty/zero defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the stage label.
    pub fn stage(mut self, v: impl Into<String>) -> Self {
        self.stage = v.into();
        self
    }

    /// Set the handler identifier.
    pub fn handler_id(mut self, v: impl Into<String>) -> Self {
        self.handler_id = v.into();
        self
    }

    /// Set the wall-clock execution duration in milliseconds.
    pub fn duration_ms(mut self, v: u64) -> Self {
        self.duration_ms = v;
        self
    }

    /// Build the [`StageCompleted`] event, stamping `occurred_at` to `SystemTime::now()`.
    pub fn build(self) -> StageCompleted {
        StageCompleted::new(self.stage, self.handler_id, self.duration_ms)
    }
}
