//! Integration test: a single `Handler::execute()` call that both reads `HandlerContext` for
//! real and, as part of the same call, dispatches a real `Query` — `Service` -> `Handler` ->
//! `Query`, connected and working, as one observable call chain.
//!
//! `AuthHandler` is a hand-composed `Handler` — not the generic `Service`->`Handler` auto-bridge
//! (`IntoHandler`/`DefaultServiceHandler` in `swe-edge-service`), which is deliberately
//! context-blind for `Service` (see `docs/3-design/dataflow.md` sections 2 and 6). `AuthHandler`
//! wraps `AuthSvc` directly and genuinely reads `req.ctx` inside its own `execute()` body, so
//! calling `auth_handler.execute(...)` once is the entire test — not two independent branches.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest, ObserverContextAdapter,
};
use edge_application_observer::StdObserveFactory;
use edge_application_query::{
    DirectQueryBus, Query, QueryBus, QueryDispatchRequest, QueryError, QueryExecuteRequest,
    QueryResultResponse,
};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
struct IsLoggedInRequest {
    session_token: String,
}
impl edge_application_service::Request for IsLoggedInRequest {}

#[derive(Debug, Clone, Copy)]
struct IsLoggedInResponse {
    logged_in: bool,
}
impl edge_application_service::Response for IsLoggedInResponse {}

/// The actual infra unit, reached only through `AuthSvc`'s own injected `QueryBus`.
struct IsLoggedInQuery {
    session_token: String,
}

impl Query for IsLoggedInQuery {
    type Result = bool;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<bool>, QueryError>> {
        let logged_in = !self.session_token.is_empty();
        Box::pin(async move { Ok(QueryResultResponse { result: logged_in }) })
    }
}

/// `Service` — holds its own injected `QueryBus`; never sees `HandlerContext`.
struct AuthSvc {
    query_bus: Arc<dyn QueryBus<Result = bool>>,
}

impl Service for AuthSvc {
    type Request = IsLoggedInRequest;
    type Response = IsLoggedInResponse;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "auth.is_logged_in".to_string(),
        })
    }

    fn execute(&self, req: IsLoggedInRequest) -> BoxFuture<'_, Result<IsLoggedInResponse, ServiceError>> {
        let bus = Arc::clone(&self.query_bus);
        Box::pin(async move {
            let result = bus
                .dispatch(QueryDispatchRequest {
                    query: Box::new(IsLoggedInQuery {
                        session_token: req.session_token,
                    }),
                })
                .await
                .map_err(|e: QueryError| ServiceError::Internal(e.to_string()))?;
            Ok(IsLoggedInResponse {
                logged_in: result.result,
            })
        })
    }
}

fn to_handler_error(err: ServiceError) -> HandlerError {
    match err {
        ServiceError::InvalidRequest(m) => HandlerError::InvalidRequest(m),
        ServiceError::RuleViolation(m) => HandlerError::FailedPrecondition(m),
        ServiceError::NotFound(m) => HandlerError::NotFound(m),
        ServiceError::Unavailable(m) => HandlerError::ExecutionFailed(m),
        ServiceError::Internal(m) => HandlerError::ExecutionFailed(m),
    }
}

/// Hand-composed `Handler`. Unlike the generic bridge, this one genuinely reads `req.ctx`.
struct AuthHandler {
    inner: AuthSvc,
}

#[async_trait]
impl Handler for AuthHandler {
    type Request = IsLoggedInRequest;
    type Response = IsLoggedInResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, IsLoggedInRequest>,
    ) -> Result<IsLoggedInResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "auth_handler".to_string(),
                message: format!("checking login for {:?}", req.req.session_token),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        // Delegates to AuthSvc, which independently dispatches its own injected Query.
        self.inner.execute(req.req).await.map_err(to_handler_error)
    }
}

fn build_ctx_and_run(
    handler: &AuthHandler,
    session_token: &str,
) -> Result<IsLoggedInResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: &observer_adapter,
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: IsLoggedInRequest {
            session_token: session_token.to_string(),
        },
        ctx: &ctx,
    }))
}

fn build_handler() -> AuthHandler {
    let query_bus: Arc<dyn QueryBus<Result = bool>> = Arc::new(DirectQueryBus::<bool>::new());
    AuthHandler {
        inner: AuthSvc { query_bus },
    }
}

/// @covers: Handler::execute
#[test]
fn test_execute_valid_token_returns_logged_in_true_happy() {
    let handler = build_handler();
    let response = build_ctx_and_run(&handler, "abc123").unwrap();
    assert!(response.logged_in);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_token_returns_logged_in_false_error() {
    let handler = build_handler();
    let response = build_ctx_and_run(&handler, "").unwrap();
    assert!(!response.logged_in);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let handler = build_handler();
    let first = build_ctx_and_run(&handler, "session-a").unwrap();
    let second = build_ctx_and_run(&handler, "").unwrap();
    let third = build_ctx_and_run(&handler, "session-b").unwrap();
    assert!(first.logged_in);
    assert!(!second.logged_in);
    assert!(third.logged_in);
}
