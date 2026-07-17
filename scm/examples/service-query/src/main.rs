//! Runnable example: a `Service` reaching real infra work through a `Query`, using ordinary
//! constructor injection — no trait changes anywhere, no deviation from the architecture.
//!
//! `AuthSvc::execute` never receives (and never needs) a caller's `HandlerContext` —
//! `Service::execute` has no context parameter, by design (see `docs/3-design/dataflow.md`
//! sections 2 and 6). It doesn't need one: the infra it "rides" is its own collaborator,
//! injected once at construction, exactly like any other hexagonal adapter dependency. Swap
//! `IsLoggedInQuery`'s body for a real session-store/auth-backend call when that's ready, and
//! `AuthSvc` itself never changes.
//!
//! Run with: `cargo run -p edge-application-service-query-example`

use std::sync::Arc;

use edge_application_query::{
    DirectQueryBus, Query, QueryBus, QueryDispatchRequest, QueryError, QueryExecuteRequest,
    QueryResultResponse,
};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
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

/// The actual infra unit — a session-store/auth-backend check, in real usage. Swappable
/// independently of `AuthSvc`; this stand-in treats any non-empty token as logged in.
struct IsLoggedInQuery {
    session_token: String,
}

impl Query for IsLoggedInQuery {
    type Result = bool;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<bool>, QueryError>> {
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
struct AuthSvc {
    query_bus: Arc<dyn QueryBus<Result = bool>>,
}

impl AuthSvc {
    fn new(query_bus: Arc<dyn QueryBus<Result = bool>>) -> Self {
        Self { query_bus }
    }
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
            println!("  [1] AuthSvc::execute — delegating to its own injected QueryBus");
            let result = bus
                .dispatch(QueryDispatchRequest {
                    query: Box::new(IsLoggedInQuery {
                        session_token: req.session_token,
                    }),
                })
                .await
                .map_err(|e: QueryError| ServiceError::Internal(e.to_string()))?;
            println!(
                "  [3] AuthSvc::execute — got {} back from the query bus",
                result.result
            );
            Ok(IsLoggedInResponse {
                logged_in: result.result,
            })
        })
    }
}

async fn check(auth_svc: &AuthSvc, session_token: &str) {
    println!(
        "[0] auth_svc.execute(IsLoggedInRequest {{ session_token: {session_token:?} }})"
    );
    match auth_svc
        .execute(IsLoggedInRequest {
            session_token: session_token.to_string(),
        })
        .await
    {
        Ok(response) => println!("[4] AuthSvc reports logged_in = {}\n", response.logged_in),
        Err(e) => println!("[4] AuthSvc returned an error: {e}\n"),
    }
}

#[tokio::main]
async fn main() {
    println!("=== AuthSvc riding infra via Query — constructor injection, no trait changes ===\n");

    let query_bus: Arc<dyn QueryBus<Result = bool>> = Arc::new(DirectQueryBus::<bool>::new());
    let auth_svc = AuthSvc::new(query_bus);

    check(&auth_svc, "abc123").await;
    check(&auth_svc, "").await;

    println!("Conclusion: AuthSvc never touched a caller's HandlerContext — it used its own,");
    println!("independently injected QueryBus to reach the actual infra (IsLoggedInQuery). Swap");
    println!("IsLoggedInQuery's body for a real session-store call and AuthSvc itself never");
    println!("changes. No trait changes anywhere in Service/Query/QueryBus were needed.");
}
