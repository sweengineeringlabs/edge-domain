//! Integration tests for the `Handler` trait contract.

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
}

struct SickHandler;
#[async_trait]
impl Handler<u32, u32> for SickHandler {
    fn id(&self) -> &str { "sick" }
    fn pattern(&self) -> &str { "sick" }
    async fn execute(&self, _: u32) -> Result<u32, HandlerError> { Err(HandlerError::Unhealthy) }
    async fn health_check(&self) -> bool { false }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_handler_trait_execute_returns_transformed_value() {
    let h = Counter { id: "ctr".into(), calls: Default::default() };
    let result = h.execute(21).await.unwrap();
    assert_eq!(result, 42);
}

/// @covers: Handler::health_check — default is true
#[tokio::test]
async fn test_handler_trait_health_check_defaults_to_true() {
    let h = Counter { id: "ctr".into(), calls: Default::default() };
    assert!(h.health_check().await);
}

/// @covers: Handler::health_check — override to false
#[tokio::test]
async fn test_handler_trait_health_check_override_returns_false() {
    let h: Arc<dyn Handler<u32, u32>> = Arc::new(SickHandler);
    assert!(!h.health_check().await);
}
