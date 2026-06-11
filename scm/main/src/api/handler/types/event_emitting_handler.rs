//! `EventEmittingHandler<H>` — wraps a handler with stage lifecycle event emission.

use std::sync::Arc;

use async_trait::async_trait;

use crate::api::event::stage::types::StageCompleted;
use crate::api::event::stage::types::StageFailed;
use crate::api::event::stage::types::StageSkipped;
use crate::api::event::stage::types::StageStarted;
use crate::api::event::DomainEvent;
use crate::api::event::EventBus;
use crate::api::handler::types::RequestContext;
use crate::api::handler::Handler;
use crate::api::handler::HandlerError;

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

async fn emit_events<Resp>(
    stage: &str,
    handler_id: &str,
    publisher: &Arc<dyn EventBus>,
    fut: impl std::future::Future<Output = Result<Resp, HandlerError>>,
) -> Result<Resp, HandlerError> {
    let _ = publisher
        .publish(Arc::new(StageStarted::new(stage, handler_id)) as Arc<dyn DomainEvent>)
        .await;

    let start = tokio::time::Instant::now();
    let result = fut.await;
    let duration_ms = start.elapsed().as_millis() as u64;

    match &result {
        Ok(_) => {
            let _ = publisher
                .publish(
                    Arc::new(StageCompleted::new(stage, handler_id, duration_ms))
                        as Arc<dyn DomainEvent>,
                )
                .await;
        }
        Err(HandlerError::Skipped) => {
            let _ = publisher
                .publish(Arc::new(StageSkipped::new(stage, handler_id)) as Arc<dyn DomainEvent>)
                .await;
        }
        Err(e) => {
            let _ = publisher
                .publish(Arc::new(StageFailed::new(
                    stage,
                    handler_id,
                    duration_ms,
                    e.to_string(),
                )) as Arc<dyn DomainEvent>)
                .await;
        }
    }

    result
}

#[async_trait]
impl<H, Req, Resp> Handler<Req, Resp> for EventEmittingHandler<H>
where
    H: Handler<Req, Resp>,
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn pattern(&self) -> &str {
        self.inner.pattern()
    }

    #[tracing::instrument(skip_all, fields(stage = %self.stage, handler_id = %self.inner.id()))]
    async fn execute(&self, req: Req) -> Result<Resp, HandlerError> {
        let handler_id = self.inner.id().to_owned();
        emit_events(
            &self.stage,
            &handler_id,
            &self.publisher,
            self.inner.execute(req),
        )
        .await
    }

    #[tracing::instrument(skip_all, fields(stage = %self.stage, handler_id = %self.inner.id()))]
    async fn execute_with_context(
        &self,
        req: Req,
        ctx: RequestContext,
    ) -> Result<Resp, HandlerError> {
        let handler_id = self.inner.id().to_owned();
        emit_events(
            &self.stage,
            &handler_id,
            &self.publisher,
            self.inner.execute_with_context(req, ctx),
        )
        .await
    }
}
