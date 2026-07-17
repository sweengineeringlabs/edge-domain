//! Integration test: the `Command` counterpart to `auth_handler_int_test.rs`. A single
//! `Handler::execute()` call that both reads `HandlerContext` for real and, as part of the
//! same call, dispatches a real `Command` — `Service` -> `Handler` -> `Command`, connected and
//! working, as one observable call chain.
//!
//! `Command::execute` returns `Result<(), CommandError>` — no payload, by design (the write
//! side of CQRS; see `docs/3-design/dataflow.md` section 6's three-tier table). So unlike the
//! `Query` version, there's no return value to assert on directly — the proof that the real
//! `Command` ran is its side effect: this test asserts against a shared store the `Command`
//! itself writes into, which is the correct way to observe a write, not a read.
#![allow(clippy::unwrap_used)]

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use edge_application_command::{
    Command as DomainCommand, CommandBus, CommandDispatchRequest, CommandError,
    DirectCommandBus, ExecutionRequest as CommandExecutionRequest,
};
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest, ObserverContextAdapter,
};
use edge_application_observer::StdObserveFactory;
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
struct RecordLoginRequest {
    session_token: String,
}
impl edge_application_service::Request for RecordLoginRequest {}

#[derive(Debug, Clone, Copy)]
struct RecordLoginResponse {
    recorded: bool,
}
impl edge_application_service::Response for RecordLoginResponse {}

/// The actual infra unit — a write, reached only through `LoginRecorderSvc`'s own
/// `CommandBus`. Writes into `store` as its observable side effect.
struct RecordLoginCommand {
    session_token: String,
    store: Arc<Mutex<Vec<String>>>,
}

impl DomainCommand for RecordLoginCommand {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>> {
        let token = self.session_token.clone();
        let store = Arc::clone(&self.store);
        Box::pin(async move {
            store.lock().unwrap().push(token);
            Ok(())
        })
    }
}

/// `Service` — holds its own injected `CommandBus`; never sees `HandlerContext`.
struct LoginRecorderSvc {
    command_bus: Arc<dyn CommandBus>,
    store: Arc<Mutex<Vec<String>>>,
}

impl Service for LoginRecorderSvc {
    type Request = RecordLoginRequest;
    type Response = RecordLoginResponse;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "auth.record_login".to_string(),
        })
    }

    fn execute(&self, req: RecordLoginRequest) -> BoxFuture<'_, Result<RecordLoginResponse, ServiceError>> {
        let bus = Arc::clone(&self.command_bus);
        let store = Arc::clone(&self.store);
        Box::pin(async move {
            bus.dispatch(CommandDispatchRequest {
                command: Box::new(RecordLoginCommand {
                    session_token: req.session_token,
                    store,
                }),
            })
            .await
            .map_err(|e: CommandError| ServiceError::Internal(e.to_string()))?;
            Ok(RecordLoginResponse { recorded: true })
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

/// Hand-composed `Handler`. Genuinely reads `req.ctx` inside its own `execute()` body.
struct LoginHandler {
    inner: LoginRecorderSvc,
}

#[async_trait]
impl Handler for LoginHandler {
    type Request = RecordLoginRequest;
    type Response = RecordLoginResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, RecordLoginRequest>,
    ) -> Result<RecordLoginResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "login_handler".to_string(),
                message: format!("recording login for {:?}", req.req.session_token),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        // Delegates to LoginRecorderSvc, which independently dispatches its own injected Command.
        self.inner.execute(req.req).await.map_err(to_handler_error)
    }
}

fn build_handler() -> (LoginHandler, Arc<Mutex<Vec<String>>>) {
    let store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let command_bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    let handler = LoginHandler {
        inner: LoginRecorderSvc {
            command_bus,
            store: Arc::clone(&store),
        },
    };
    (handler, store)
}

fn run(handler: &LoginHandler, session_token: &str) -> Result<RecordLoginResponse, HandlerError> {
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
        req: RecordLoginRequest {
            session_token: session_token.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_valid_token_writes_to_store_happy() {
    let (handler, store) = build_handler();
    let response = run(&handler, "abc123").unwrap();
    assert!(response.recorded);
    assert_eq!(store.lock().unwrap().as_slice(), ["abc123"]);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_token_still_writes_recorded_entry_error() {
    let (handler, store) = build_handler();
    let response = run(&handler, "").unwrap();
    assert!(response.recorded);
    assert_eq!(store.lock().unwrap().as_slice(), [""]);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_accumulate_in_store_edge() {
    let (handler, store) = build_handler();
    run(&handler, "session-a").unwrap();
    run(&handler, "session-b").unwrap();
    run(&handler, "session-c").unwrap();
    assert_eq!(
        store.lock().unwrap().as_slice(),
        ["session-a", "session-b", "session-c"]
    );
}
