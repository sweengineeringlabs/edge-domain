//! Integration test: `ResolveGreeterHandler::execute()` reads through a real,
//! constructor-injected `Registry` — the same instance a `RegisterGreeterHandler` writes into,
//! proving one registry can genuinely back multiple handlers, each reaching it through its own
//! injected field rather than through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_registry::{
    MemoryRegistry, Registry, RegistryLookupRequest, TryRegisterRequest,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}

struct EnglishGreeter;
impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}

#[derive(Debug, Clone)]
struct ResolveGreeterRequest {
    id: String,
    name: String,
}
impl edge_application_base::Request for ResolveGreeterRequest {}

#[derive(Debug, Clone)]
struct ResolveGreeterResponse {
    greeting: Option<String>,
}
impl edge_application_base::Response for ResolveGreeterResponse {}

/// Holds its own injected `Registry` — the per-call lookup id arrives through `Self::Request`.
struct ResolveGreeterHandler {
    registry: Arc<dyn Registry<Value = dyn Greeter>>,
}

#[async_trait]
impl Handler for ResolveGreeterHandler {
    type Request = ResolveGreeterRequest;
    type Response = ResolveGreeterResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, ResolveGreeterRequest>,
    ) -> Result<ResolveGreeterResponse, HandlerError> {
        let resolved = self
            .registry
            .get(RegistryLookupRequest { id: req.req.id })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .entry;
        Ok(ResolveGreeterResponse {
            greeting: resolved.map(|greeter| greeter.greet(&req.req.name)),
        })
    }
}

/// Seeds a registry with one greeter and builds a handler sharing that same instance — mirrors
/// the multi-handler-one-registry shape from `RegisterGreeterHandler`.
fn build_handler_with_seeded_greeter() -> ResolveGreeterHandler {
    let registry: Arc<dyn Registry<Value = dyn Greeter>> =
        Arc::new(MemoryRegistry::<dyn Greeter>::new());
    registry
        .try_register(TryRegisterRequest {
            id: "en-greeter".to_string(),
            entry: Arc::new(EnglishGreeter) as Arc<dyn Greeter>,
        })
        .unwrap();
    ResolveGreeterHandler { registry }
}

fn run(
    handler: &ResolveGreeterHandler,
    id: &str,
    name: &str,
) -> Result<ResolveGreeterResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: ResolveGreeterRequest {
            id: id.to_string(),
            name: name.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_registered_id_returns_greeting_happy() {
    let handler = build_handler_with_seeded_greeter();
    let response = run(&handler, "en-greeter", "Ada").unwrap();
    assert_eq!(response.greeting, Some("Hello, Ada!".to_string()));
}

/// @covers: Handler::execute
#[test]
fn test_execute_unregistered_id_returns_none_error() {
    let handler = build_handler_with_seeded_greeter();
    let response = run(&handler, "does-not-exist", "Ada").unwrap();
    assert_eq!(response.greeting, None);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_and_consistent_edge() {
    let handler = build_handler_with_seeded_greeter();
    let first = run(&handler, "en-greeter", "Ada").unwrap();
    let second = run(&handler, "missing", "Ada").unwrap();
    let third = run(&handler, "en-greeter", "Ada").unwrap();
    assert!(first.greeting.is_some());
    assert!(second.greeting.is_none());
    assert_eq!(first.greeting, third.greeting);
}
