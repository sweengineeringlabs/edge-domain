//! `StageCompleted` — emitted when a pipeline stage finishes successfully.

use std::time::SystemTime;

use crate::api::event::DomainEvent;

/// Emitted by [`EventEmittingHandler`](crate::EventEmittingHandler) after the inner
/// handler returns `Ok(_)`.
pub struct StageCompleted {
    kind: &'static str,
    stage: String,
    handler_id: String,
    duration_ms: u64,
    occurred_at: SystemTime,
}

impl StageCompleted {
    /// Construct a new `StageCompleted` event.
    pub fn new(stage: impl Into<String>, handler_id: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            kind: "stage.completed",
            stage: stage.into(),
            handler_id: handler_id.into(),
            duration_ms,
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

    /// Wall-clock execution time in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }
}

impl DomainEvent for StageCompleted {
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
