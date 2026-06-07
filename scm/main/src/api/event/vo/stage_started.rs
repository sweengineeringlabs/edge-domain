//! `StageStarted` — emitted when a pipeline stage begins execution.

use std::time::SystemTime;

use crate::api::event::DomainEvent;

/// Emitted by [`EventEmittingHandler`](crate::EventEmittingHandler) immediately
/// before delegating to the inner handler.
///
/// Consumers use this to open a tracing span or start a latency timer.
pub struct StageStarted {
    stage: String,
    handler_id: String,
    occurred_at: SystemTime,
}

impl StageStarted {
    /// Construct a new `StageStarted` event.
    pub fn new(stage: impl Into<String>, handler_id: impl Into<String>) -> Self {
        Self {
            stage: stage.into(),
            handler_id: handler_id.into(),
            occurred_at: SystemTime::now(),
        }
    }

    /// The stage label assigned at [`EventEmittingHandler`](crate::EventEmittingHandler) construction.
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// The [`Handler::id`](crate::Handler::id) of the wrapped handler.
    pub fn handler_id(&self) -> &str {
        &self.handler_id
    }
}

impl DomainEvent for StageStarted {
    fn event_type(&self) -> &str {
        "stage.started"
    }

    fn aggregate_id(&self) -> &str {
        &self.stage
    }

    fn occurred_at(&self) -> SystemTime {
        self.occurred_at
    }
}
