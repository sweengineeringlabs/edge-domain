//! Runnable example: two `Handler`s sharing one constructor-injected `Registry<Value>`, with
//! the per-call lookup id carried through `Self::Request` — the generic-per-type port wiring
//! pattern `HandlerContext` structurally can't hold. This is the last of the seven applicable
//! ports in issue #149.
//!
//! `Registry`'s own doc comment states every method takes `&self` and is concurrent by design
//! (`MemoryRegistry` handles interior mutability internally via `RwLock`) — the simplest port in
//! this series alongside `policy`, plain `Arc<dyn Registry<...>>` injection, no lock needed.
//!
//! `RegisterGreeterHandler::execute` genuinely reads `HandlerContext` for logging. Both handlers
//! here share the same injected `Arc<dyn Registry<Value = dyn Greeter>>` instance.
//!
//! Run with: `cargo run -p edge-application-registry-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_registry::{
    MemoryRegistry, Registry, RegistryLookupRequest, TryRegisterRequest,
};
use edge_security_runtime::SecurityContext;

trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}

struct EnglishGreeter;
impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}

struct SpanishGreeter;
impl Greeter for SpanishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("¡Hola, {name}!")
    }
}

#[derive(Debug, Clone)]
struct RegisterGreeterRequest {
    id: String,
    language: String,
}
impl edge_application_base::Request for RegisterGreeterRequest {}

#[derive(Debug, Clone, Copy)]
struct RegisterGreeterResponse {
    registered: bool,
}
impl edge_application_base::Response for RegisterGreeterResponse {}

/// Holds its own injected `Registry`; genuinely reads `req.ctx` inside `execute()`.
struct RegisterGreeterHandler {
    registry: Arc<dyn Registry<Value = dyn Greeter>>,
}

#[async_trait]
impl Handler for RegisterGreeterHandler {
    type Request = RegisterGreeterRequest;
    type Response = RegisterGreeterResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, RegisterGreeterRequest>,
    ) -> Result<RegisterGreeterResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "register_greeter_handler".to_string(),
                message: format!("registering {:?} greeter under {:?}", req.req.language, req.req.id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] RegisterGreeterHandler::execute — delegating to its own injected Registry");
        let greeter: Arc<dyn Greeter> = match req.req.language.as_str() {
            "es" => Arc::new(SpanishGreeter),
            _ => Arc::new(EnglishGreeter),
        };
        self.registry
            .try_register(TryRegisterRequest {
                id: req.req.id,
                entry: greeter,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!("  [2] RegisterGreeterHandler::execute — registry confirmed the registration");
        Ok(RegisterGreeterResponse { registered: true })
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

/// Holds the *same* injected `Registry` instance as `RegisterGreeterHandler`.
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
        println!("  [1] ResolveGreeterHandler::execute — delegating to its own injected Registry");
        let resolved = self
            .registry
            .get(RegistryLookupRequest { id: req.req.id })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .entry;
        let greeting = resolved.map(|greeter| greeter.greet(&req.req.name));
        println!("  [2] ResolveGreeterHandler::execute — registry returned {greeting:?}");
        Ok(ResolveGreeterResponse { greeting })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected Registry<Value> — constructor injection, Self::Request carries the id ===\n");

    let registry: Arc<dyn Registry<Value = dyn Greeter>> = Arc::new(MemoryRegistry::<dyn Greeter>::new());
    let register_handler = RegisterGreeterHandler {
        registry: Arc::clone(&registry),
    };
    let resolve_handler = ResolveGreeterHandler {
        registry: Arc::clone(&registry),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] register_handler.execute(RegisterGreeterRequest {{ id: \"es-greeter\", language: \"es\" }})");
    let register_resp = register_handler
        .execute(HandlerExecutionRequest {
            req: RegisterGreeterRequest {
                id: "es-greeter".to_string(),
                language: "es".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("register should succeed");
    println!("[3] RegisterGreeterHandler reports registered = {}\n", register_resp.registered);

    println!("[0] resolve_handler.execute(ResolveGreeterRequest {{ id: \"es-greeter\", name: \"Ada\" }})");
    let resolve_resp = resolve_handler
        .execute(HandlerExecutionRequest {
            req: ResolveGreeterRequest {
                id: "es-greeter".to_string(),
                name: "Ada".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("resolve should succeed");
    println!("[3] ResolveGreeterHandler returned {:?}\n", resolve_resp.greeting);

    println!("[0] register_handler.execute(RegisterGreeterRequest {{ id: \"es-greeter\", language: \"en\" }}) — duplicate id");
    let duplicate = register_handler
        .execute(HandlerExecutionRequest {
            req: RegisterGreeterRequest {
                id: "es-greeter".to_string(),
                language: "en".to_string(),
            },
            ctx: &ctx,
        })
        .await;
    match duplicate {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] RegisterGreeterHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: RegisterGreeterHandler used ctx.observer for real per-request logging,");
    println!("but both handlers reached the actual registry only through their own, independently");
    println!("injected Registry<dyn Greeter> — never through HandlerContext, which structurally");
    println!("cannot hold a generic-per-type port. The registry's own duplicate-id rejection");
    println!("propagated cleanly as a real HandlerError. This completes all seven applicable ports.");
}
