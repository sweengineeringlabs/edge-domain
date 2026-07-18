//! Runnable trace of where `HandlerContext` does — and does not — reach, across the
//! `Handler` -> `Service` -> `Command` boundary documented in `docs/3-design/dataflow.md`
//! sections 2 and 6.
//!
//! This crate exists purely to make that finding observable at the code level, not to add a
//! new production bridge. `DemoServiceHandler` below reproduces the same shape as
//! `swe-edge-service`'s `DefaultServiceHandler` (external to this repo — see dataflow.md
//! section 2's 2026-07-17 amendment). `domain-handler` itself carries no dependency on
//! `domain-service` (confirmed by `domain-handler/Cargo.toml`, restored by issue #143), so this
//! bridge is built once, locally, for tracing only.
//!
//! **Important:** the two branches below (`Handler -> Command` and `Handler -> Service`) are
//! independent calls made directly by `Handler::execute`, run one after the other in this file's
//! source order. Neither branch is downstream of the other — `Command::execute` never calls
//! `Service::execute` and vice versa. Printing them in sequence is *not* evidence of a pipeline;
//! `UnreachableServiceCommand` below exists specifically to show, from inside a `Command`'s own
//! body, that it has nothing available to reach a `Service` with even if it wanted to.
//!
//! Run with: `cargo run -p edge-application-dataflow-example`

use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use edge_application_base::{Request as BaseRequest, Response as BaseResponse};
use edge_application_command::{
    Command as DomainCommand, CommandError, DirectCommandBus,
    ExecutionRequest as CommandExecutionRequest,
};
use edge_application_handler::{
    CommandDispatchRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_service::{Service, ServiceError};
use edge_security_runtime::SecurityContext;
use futures::future::BoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Payload(String);

impl BaseRequest for Payload {}
impl BaseResponse for Payload {}

/// A `Service` impl — deliberately context-blind, per `domain-service`'s own trait shape.
struct EchoService;

impl Service for EchoService {
    type Request = Payload;
    type Response = Payload;

    fn execute(&self, req: Payload) -> BoxFuture<'_, Result<Payload, ServiceError>> {
        println!("  [B2] Service::execute({:?}) running now", req.0);
        println!(
            "       -> this fn's own signature has no context parameter at all: there is"
        );
        println!("          nothing named `ctx` in scope here, by construction, not omission.");
        Box::pin(async move { Ok(Payload(format!("echo: {}", req.0))) })
    }
}

/// A `Command` whose entire purpose is to demonstrate, from inside its own body, that it has
/// no way to reach `EchoService` (or any `Service`) — not "didn't bother to," but "cannot,"
/// per `Command::execute`'s own signature. See `docs/3-design/dataflow.md` section 6.
struct UnreachableServiceCommand;

impl DomainCommand for UnreachableServiceCommand {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>> {
        Box::pin(async {
            println!("  [A2] Command::execute running");
            println!("       -> to invoke EchoService from here, I would need either a");
            println!("          ServiceRegistry reference (to look it up by name) or a direct");
            println!("          Service reference.");
            println!(
                "       -> I have neither: this fn's full parameter list is `(&self, _req:"
            );
            println!(
                "          ExecutionRequest)`, and ExecutionRequest is a zero-sized unit struct"
            );
            println!("          — no payload, no ctx, nothing to reach a Service with.");
            println!("       -> this is not a missing feature I chose not to use; there is no");
            println!("          parameter here that could carry such a reference in the first place.");
            Ok(())
        })
    }
}

/// Reproduces `swe-edge-service`'s `DefaultServiceHandler<S>` shape locally, for tracing only.
/// See `docs/3-design/dataflow.md` section 2 — the real, production bridge lives in that
/// external repo, not here.
struct DemoServiceHandler<S> {
    inner: S,
}

impl<S> DemoServiceHandler<S> {
    fn new(inner: S) -> Self {
        Self { inner }
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

#[async_trait]
impl<S> Handler for DemoServiceHandler<S>
where
    S: Service + Send + Sync,
    S::Request: BaseRequest + Send + 'static,
    S::Response: BaseResponse + Send + 'static,
{
    type Request = S::Request;
    type Response = S::Response;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, S::Request>,
    ) -> Result<S::Response, HandlerError> {
        println!("[0] Handler::execute received ExecutionRequest {{ req, ctx }}");
        println!("    -> ctx IS in scope here: security/commands/observer all reachable.");
        println!(
            "    -> the two branches below are independent calls this fn makes directly;"
        );
        println!("       neither branch calls the other.\n");

        println!("--- branch A: Handler -> Command (ctx.commands.dispatch) ---");
        println!("  [A1] dispatching UnreachableServiceCommand through ctx.commands");
        req.ctx
            .commands
            .dispatch(CommandDispatchRequest {
                command: Box::new(UnreachableServiceCommand),
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(format!("command dispatch failed: {e}")))?;

        println!("\n--- branch B: Handler -> Service (self.inner.execute) — produces the response ---");
        println!("  [B1] forwarding req.req to Service::execute — only the payload crosses this");
        println!("       line; req.ctx is never read past it.");
        self.inner.execute(req.req).await.map_err(to_handler_error)
    }
}

#[tokio::main]
async fn main() {
    println!("=== Tracing HandlerContext across Handler -> Command and Handler -> Service ===\n");

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    let handler = DemoServiceHandler::new(EchoService);

    let result = handler
        .execute(HandlerExecutionRequest {
            req: Payload("hello".to_string()),
            ctx: &ctx,
        })
        .await;

    println!("\n[1] final result: {result:?}");
    println!(
        "\nConclusion: branch A (Handler -> Command) and branch B (Handler -> Service) are"
    );
    println!("independent — Command never calls Service, Service never calls Command. ctx reaches");
    println!("branch A's Command (via ctx.commands.dispatch) but never reaches branch B's Service");
    println!("(no parameter exists to carry it). There is no code path, anywhere in this program");
    println!("or in the real crates it mirrors, connecting a dispatched Command to a named");
    println!("Service. See docs/3-design/dataflow.md sections 2 and 6 for the written trace.");
}
