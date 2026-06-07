//! `EventEmittingHandler<H>` — wraps a handler with stage lifecycle event emission.

use std::sync::Arc;

use crate::api::event::EventBus;

/// A [`Handler`](crate::Handler) decorator that publishes stage lifecycle events
/// to an [`EventBus`] before and after each call to the inner handler.
///
/// Emits:
/// - [`StageStarted`](crate::StageStarted) — before delegating to inner
/// - [`StageCompleted`](crate::StageCompleted) — after `Ok(_)` from inner
/// - [`StageFailed`](crate::StageFailed) — after `Err(e)` where `e` is not `Skipped`
/// - [`StageSkipped`](crate::StageSkipped) — after [`HandlerError::Skipped`](crate::HandlerError)
///
/// Also opens a `tracing` span per call, tagged with `stage` and `handler_id`.
///
/// # Examples
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use async_trait::async_trait;
/// use edge_domain::{Domain, EventEmittingHandler, Handler, HandlerError};
///
/// struct PingHandler;
/// #[async_trait]
/// impl Handler<String, String> for PingHandler {
///     fn id(&self) -> &str { "ping" }
///     fn pattern(&self) -> &str { "/ping" }
///     async fn execute(&self, req: String) -> Result<String, HandlerError> { Ok(req) }
/// }
///
/// let bus = Domain::in_process_event_bus(Default::default());
/// let handler = EventEmittingHandler::new(PingHandler, bus, "cache");
/// ```
pub struct EventEmittingHandler<H> {
    pub(crate) inner: H,
    pub(crate) publisher: Arc<dyn EventBus>,
    pub(crate) stage: String,
}

impl<H> EventEmittingHandler<H> {
    /// Wrap `inner` with event emission under the given `stage` label.
    ///
    /// `stage` is used as the label in all emitted events and as the tracing span field.
    pub fn new(inner: H, publisher: Arc<dyn EventBus>, stage: impl Into<String>) -> Self {
        Self {
            inner,
            publisher,
            stage: stage.into(),
        }
    }

    /// The stage label assigned at construction.
    pub fn stage(&self) -> &str {
        &self.stage
    }
}
