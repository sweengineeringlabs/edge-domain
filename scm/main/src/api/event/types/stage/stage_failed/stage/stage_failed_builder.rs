//! `StageFailedBuilder` — fluent constructor for [`StageFailed`](super::StageFailed).

use crate::api::event::types::stage::StageFailed;

/// Fluent builder for [`StageFailed`] events.
///
/// `occurred_at` is stamped to `SystemTime::now()` at [`build`](Self::build) time,
/// matching the behaviour of [`StageFailed::new`].
#[derive(Default)]
pub struct StageFailedBuilder {
    stage: String,
    handler_id: String,
    duration_ms: u64,
    error: String,
}

impl StageFailedBuilder {
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

    /// Set the human-readable error description.
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.error = v.into();
        self
    }

    /// Build the [`StageFailed`] event, stamping `occurred_at` to `SystemTime::now()`.
    pub fn build(self) -> StageFailed {
        StageFailed::new(self.stage, self.handler_id, self.duration_ms, self.error)
    }
}
