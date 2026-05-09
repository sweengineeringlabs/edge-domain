//! Integration tests for the `Handler` trait contract.

use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError};

struct Counter { id: String, calls: std::sync::atomic::AtomicUsize }

#[async_trait]
impl Handler<u32, u32> for Counter {
    fn id(&self) -> &str { &self.id }
    fn pattern(&self) -> &str { "counter" }
    async fn execute(&self, req: u32) -> Result<u32, HandlerError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(req * 2)
    }
    async fn health_check(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_handler_trait_execute_returns_transformed_value() {
    let h = Counter { id: "ctr".into(), calls: Default::default() };
    let result = h.execute(21).await.unwrap();
    assert_eq!(result, 42);
}

/// @covers: Handler::health_check
#[tokio::test]
async fn test_handler_trait_health_check_returns_true_for_healthy_handler() {
    let h = Counter { id: "ctr".into(), calls: Default::default() };
    assert!(h.health_check().await);
}

/// @covers: Handler::as_any
#[tokio::test]
async fn test_handler_trait_as_any_supports_downcast_to_concrete_type() {
    let h: Arc<dyn Handler<u32, u32>> =
        Arc::new(Counter { id: "ctr".into(), calls: Default::default() });
    assert!(h.as_any().downcast_ref::<Counter>().is_some());
}
