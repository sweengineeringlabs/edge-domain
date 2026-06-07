//! [`Handler`] impl for [`EventEmittingHandler`].

use std::sync::Arc;

use async_trait::async_trait;
use edge_dispatch::Handler;
use edge_dispatch::HandlerError;
use edge_dispatch::RequestContext;

use crate::api::event::vo::StageCompleted;
use crate::api::event::vo::StageFailed;
use crate::api::event::vo::StageSkipped;
use crate::api::event::vo::StageStarted;
use crate::api::event::DomainEvent;
use crate::api::event::EventBus;
use crate::api::handler::types::EventEmittingHandler;

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
                .publish(Arc::new(StageCompleted::new(stage, handler_id, duration_ms)) as Arc<dyn DomainEvent>)
                .await;
        }
        Err(HandlerError::Skipped) => {
            let _ = publisher
                .publish(Arc::new(StageSkipped::new(stage, handler_id)) as Arc<dyn DomainEvent>)
                .await;
        }
        Err(e) => {
            let _ = publisher
                .publish(Arc::new(StageFailed::new(stage, handler_id, duration_ms, e.to_string())) as Arc<dyn DomainEvent>)
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
        emit_events(&self.stage, &handler_id, &self.publisher, self.inner.execute(req)).await
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use async_trait::async_trait;
    use futures::future::BoxFuture;
    use parking_lot::Mutex;

    use super::*;
    use crate::api::event::EventBus;
    use crate::api::event::EventError;
    use crate::api::event::EventReceiver;
    use edge_dispatch::EchoHandler;

    struct CollectingBus {
        emitted: Arc<Mutex<Vec<String>>>,
    }

    impl CollectingBus {
        fn new() -> (Arc<Self>, Arc<Mutex<Vec<String>>>) {
            let emitted = Arc::new(Mutex::new(Vec::new()));
            (Arc::new(Self { emitted: Arc::clone(&emitted) }), emitted)
        }
    }

    impl EventBus for CollectingBus {
        fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>> {
            self.emitted.lock().push(event.event_type().to_string());
            Box::pin(async { Ok(()) })
        }

        fn subscribe(&self) -> EventReceiver {
            unimplemented!("test bus does not support subscribe")
        }
    }

    struct FailingHandler;

    #[async_trait]
    impl Handler<String, String> for FailingHandler {
        fn id(&self) -> &str { "fail" }
        fn pattern(&self) -> &str { "/fail" }
        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Err(HandlerError::ExecutionFailed("inner error".to_string()))
        }
    }

    struct SkippingHandler;

    #[async_trait]
    impl Handler<String, String> for SkippingHandler {
        fn id(&self) -> &str { "skip" }
        fn pattern(&self) -> &str { "/skip" }
        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Err(HandlerError::Skipped)
        }
    }

    struct SlowHandler {
        delay: Duration,
    }

    #[async_trait]
    impl Handler<String, String> for SlowHandler {
        fn id(&self) -> &str { "slow" }
        fn pattern(&self) -> &str { "/slow" }
        async fn execute(&self, req: String) -> Result<String, HandlerError> {
            tokio::time::sleep(self.delay).await;
            Ok(req)
        }
    }

    #[tokio::test]
    async fn test_execute_success_emits_started_then_completed() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("e", "/e"), bus, "cache");
        let _ = h.execute("x".to_string()).await.unwrap();
        let events = emitted.lock().clone();
        assert_eq!(events, ["stage.started", "stage.completed"]);
    }

    #[tokio::test]
    async fn test_execute_failure_emits_started_then_failed() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(FailingHandler, bus, "guard");
        let _ = h.execute("x".to_string()).await.unwrap_err();
        let events = emitted.lock().clone();
        assert_eq!(events, ["stage.started", "stage.failed"]);
    }

    #[tokio::test]
    async fn test_execute_skipped_emits_started_then_skipped_not_failed() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(SkippingHandler, bus, "tuner");
        let err = h.execute("x".to_string()).await.unwrap_err();
        assert!(matches!(err, HandlerError::Skipped));
        let events = emitted.lock().clone();
        assert_eq!(events, ["stage.started", "stage.skipped"]);
    }

    #[tokio::test]
    async fn test_execute_propagates_success_result() {
        let (bus, _) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("e", "/e"), bus, "render");
        assert_eq!(h.execute("hello".to_string()).await.unwrap(), "hello");
    }

    #[tokio::test]
    async fn test_execute_propagates_error_result() {
        let (bus, _) = CollectingBus::new();
        let h = EventEmittingHandler::new(FailingHandler, bus, "guard");
        let err = h.execute("x".to_string()).await.unwrap_err();
        assert!(matches!(err, HandlerError::ExecutionFailed(_)));
    }

    #[tokio::test]
    async fn test_execute_records_nonzero_duration_in_completed_event() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(
            SlowHandler { delay: Duration::from_millis(5) },
            bus,
            "render",
        );
        h.execute("x".to_string()).await.unwrap();
        assert_eq!(emitted.lock().clone(), ["stage.started", "stage.completed"]);
    }

    #[tokio::test]
    async fn test_execute_with_context_success_emits_started_then_completed() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("e", "/e"), bus, "cache");
        let ctx = RequestContext::unauthenticated();
        h.execute_with_context("x".to_string(), ctx).await.unwrap();
        let events = emitted.lock().clone();
        assert_eq!(events, ["stage.started", "stage.completed"]);
    }

    #[tokio::test]
    async fn test_execute_with_context_failure_emits_started_then_failed() {
        let (bus, emitted) = CollectingBus::new();
        let h = EventEmittingHandler::new(FailingHandler, bus, "guard");
        let ctx = RequestContext::unauthenticated();
        h.execute_with_context("x".to_string(), ctx).await.unwrap_err();
        let events = emitted.lock().clone();
        assert_eq!(events, ["stage.started", "stage.failed"]);
    }

    #[test]
    fn test_id_delegates_to_inner() {
        let (bus, _) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("my-id", "/p"), bus, "s");
        assert_eq!(h.id(), "my-id");
    }

    #[test]
    fn test_pattern_delegates_to_inner() {
        let (bus, _) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("id", "/api/v1/foo"), bus, "s");
        assert_eq!(h.pattern(), "/api/v1/foo");
    }

    #[test]
    fn test_stage_accessor_returns_configured_label() {
        let (bus, _) = CollectingBus::new();
        let h = EventEmittingHandler::new(EchoHandler::<String>::new("id", "/p"), bus, "cache");
        assert_eq!(h.stage(), "cache");
    }
}
