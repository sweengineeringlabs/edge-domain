//! Runnable example: a `Handler` reaching real infra work through a `Query`, using ordinary
//! constructor injection — no trait changes anywhere, no deviation from the architecture.
//!
//! `AuthHandler::execute` genuinely reads `HandlerContext` (it emits a log record through
//! `ctx.observer` on every call), but the actual login check doesn't need it: the `QueryBus` it
//! rides is its own collaborator, injected once at construction, exactly like any other
//! hexagonal adapter dependency. Swap `IsLoggedInQuery`'s body for a real
//! session-store/auth-backend call when that's ready, and `AuthHandler` itself never changes.
//!
//! (Formerly demonstrated this pattern through a `Service` wrapped by a hand-composed `Handler`
//! — `Service`/`ServiceRegistry` were removed as redundant with `Handler`/`HandlerRegistry`, see
//! issue #147. `Handler` alone now carries the same "constructor-injected collaborator, context
//! available but not required" story directly.)
//!
//! Run with: `cargo run -p edge-application-service-query-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_query::{
    DirectQueryBus, Query, QueryBus, QueryDispatchRequest, QueryError, QueryExecuteRequest,
    QueryResultResponse,
};
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone)]
struct IsLoggedInRequest {
    session_token: String,
}
impl edge_application_base::Request for IsLoggedInRequest {}

#[derive(Debug, Clone, Copy)]
struct IsLoggedInResponse {
    logged_in: bool,
}
impl edge_application_base::Response for IsLoggedInResponse {}

/// The actual infra unit — a session-store/auth-backend check, in real usage. Swappable
/// independently of `AuthHandler`; this stand-in treats any non-empty token as logged in.
struct IsLoggedInQuery {
    session_token: String,
}

impl Query for IsLoggedInQuery {
    type Result = bool;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> futures::future::BoxFuture<'_, Result<QueryResultResponse<bool>, QueryError>> {
        let token = self.session_token.clone();
        Box::pin(async move {
            println!(
                "    [infra] IsLoggedInQuery::execute — checking session store for {token:?}"
            );
            let logged_in = !token.is_empty();
            Ok(QueryResultResponse { result: logged_in })
        })
    }
}

/// The port other domain logic calls. Holds its own `QueryBus`, injected once at construction.
struct AuthHandler {
    query_bus: Arc<dyn QueryBus<Result = bool>>,
}

impl AuthHandler {
    fn new(query_bus: Arc<dyn QueryBus<Result = bool>>) -> Self {
        Self { query_bus }
    }
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

        println!("  [1] AuthHandler::execute — delegating to its own injected QueryBus");
        let result = self
            .query_bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(IsLoggedInQuery {
                    session_token: req.req.session_token,
                }),
            })
            .await
            .map_err(|e: QueryError| HandlerError::ExecutionFailed(e.to_string()))?;
        println!(
            "  [3] AuthHandler::execute — got {} back from the query bus",
            result.result
        );
        Ok(IsLoggedInResponse {
            logged_in: result.result,
        })
    }
}

async fn check(auth_handler: &AuthHandler, ctx: &HandlerContext<'_>, session_token: &str) {
    println!("[0] auth_handler.execute(IsLoggedInRequest {{ session_token: {session_token:?} }})");
    match auth_handler
        .execute(HandlerExecutionRequest {
            req: IsLoggedInRequest {
                session_token: session_token.to_string(),
            },
            ctx,
        })
        .await
    {
        Ok(response) => println!("[4] AuthHandler reports logged_in = {}\n", response.logged_in),
        Err(e) => println!("[4] AuthHandler returned an error: {e}\n"),
    }
}

#[tokio::main]
async fn main() {
    println!("=== AuthHandler riding infra via Query — constructor injection, no trait changes ===\n");

    let query_bus: Arc<dyn QueryBus<Result = bool>> = Arc::new(DirectQueryBus::<bool>::new());
    let auth_handler = AuthHandler::new(query_bus);

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    check(&auth_handler, &ctx, "abc123").await;
    check(&auth_handler, &ctx, "").await;

    println!("Conclusion: AuthHandler used ctx.observer for real per-request logging, but its own,");
    println!("independently injected QueryBus to reach the actual infra (IsLoggedInQuery) never");
    println!("needed HandlerContext at all. Swap IsLoggedInQuery's body for a real session-store");
    println!("call and AuthHandler itself never changes. No trait changes anywhere in");
    println!("Handler/Query/QueryBus were needed.");
}
