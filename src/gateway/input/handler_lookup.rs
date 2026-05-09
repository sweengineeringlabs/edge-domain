//! Inbound handler lookup adapter.

use std::sync::Arc;

use crate::api::handler::Handler;
use crate::api::handler::handler_registry::HandlerRegistry;

/// Look up a handler by id from the given registry.
pub fn find_handler<Req, Resp>(
    registry: &HandlerRegistry<Req, Resp>,
    id: &str,
) -> Option<Arc<dyn Handler<Req, Resp>>>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    registry.get(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler::request_context::RequestContext;
    use crate::api::handler_error::HandlerError;
    use async_trait::async_trait;
    
    struct HandlerLookupStub;
    #[async_trait]
    impl Handler<String, String> for HandlerLookupStub {
        fn id(&self) -> &str { "stub" }
        fn pattern(&self) -> &str { "stub" }
        async fn execute(&self, req: String) -> Result<String, HandlerError> { Ok(req) }
    }

    /// @covers: find_handler
    #[test]
    fn test_find_handler_returns_none_for_missing_id() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        assert!(find_handler(&reg, "missing").is_none());
    }

    /// @covers: find_handler
    #[test]
    fn test_find_handler_returns_some_for_registered_id() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        reg.register(Arc::new(HandlerLookupStub));
        assert!(find_handler(&reg, "stub").is_some());
    }
}
