//! `StageFailed` — emitted when a pipeline stage returns a non-skipped error.

use std::time::SystemTime;

use crate::api::event::DomainEvent;

/// Emitted by [`EventEmittingHandler`](crate::EventEmittingHandler) after the inner
/// handler returns `Err(e)` where `e` is not
/// [`HandlerError::Skipped`](edge_dispatch::HandlerError::Skipped).
///
/// For `Skipped`, see [`StageSkipped`](crate::StageSkipped) instead.
pub struct StageFailed {
    kind: &'static str,
    stage: String,
    handler_id: String,
    duration_ms: u64,
    error: String,
    occurred_at: SystemTime,
}

impl StageFailed {
    /// Construct a new `StageFailed` event.
    pub fn new(
        stage: impl Into<String>,
        handler_id: impl Into<String>,
        duration_ms: u64,
        error: impl Into<String>,
    ) -> Self {
        Self {
            kind: "stage.failed",
            stage: stage.into(),
            handler_id: handler_id.into(),
            duration_ms,
            error: error.into(),
            occurred_at: SystemTime::now(),
        }
    }

    /// The stage label.
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// The [`Handler::id`](crate::Handler::id) of the wrapped handler.
    pub fn handler_id(&self) -> &str {
        &self.handler_id
    }

    /// Wall-clock execution time in milliseconds (until the error was returned).
    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    /// Human-readable error description from [`HandlerError::to_string`](edge_dispatch::HandlerError).
    pub fn error(&self) -> &str {
        &self.error
    }
}

impl DomainEvent for StageFailed {
    fn event_type(&self) -> &str {
        self.kind
    }

    fn aggregate_id(&self) -> &str {
        &self.stage
    }

    fn occurred_at(&self) -> SystemTime {
        self.occurred_at
    }
}
