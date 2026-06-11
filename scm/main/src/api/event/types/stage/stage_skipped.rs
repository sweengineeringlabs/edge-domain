//! `StageSkipped` — emitted when a pipeline stage is bypassed via `HandlerError::Skipped`.

use std::time::SystemTime;

use crate::api::event::DomainEvent;

/// Emitted by [`EventEmittingHandler`](crate::EventEmittingHandler) after the inner
/// handler returns [`HandlerError::Skipped`](edge_dispatch::HandlerError::Skipped).
///
/// `Skipped` is a pipeline-internal continuation signal, not a real failure.
/// RFC-001 `Pipeline` treats it as "continue to next stage"; observability
/// consumers treat it as a non-error bypass rather than a fault.
pub struct StageSkipped {
    kind: &'static str,
    stage: String,
    handler_id: String,
    occurred_at: SystemTime,
}

impl StageSkipped {
    /// Construct a new `StageSkipped` event.
    pub fn new(stage: impl Into<String>, handler_id: impl Into<String>) -> Self {
        Self {
            kind: "stage.skipped",
            stage: stage.into(),
            handler_id: handler_id.into(),
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
}

impl DomainEvent for StageSkipped {
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
