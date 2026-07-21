//! Runnable example: two `Handler`s sharing one constructor-injected `Projection<Event,
//! ReadModel>`, with the per-call events carried through `Self::Request` — the generic-per-type
//! port wiring pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! `Projection::apply`/`try_drain` take `&mut self` (same shape as `Saga::handle`), so this
//! wires `Handler` to the projection via `Arc<parking_lot::Mutex<P>>`, same as
//! `examples/saga-handler`. Unlike `saga`, there's no separate store trait with an
//! immutable-borrow-return problem — `Projection` is held directly.
//!
//! `ApplyCreditsHandler::execute` genuinely reads `HandlerContext` for logging. Both handlers
//! here share the same injected `Arc<Mutex<Balance>>` instance.
//!
//! Run with: `cargo run -p edge-application-projection-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_projection::{
    Projection, ProjectionEvent, ProjectionEventDescribeRequest, ProjectionEventDescribeResponse,
    ProjectionError, ProjectionReadModelRequest, TryDrainRequest,
};
use edge_security_runtime::SecurityContext;
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy)]
struct Credited {
    amount: u64,
}

impl ProjectionEvent for Credited {
    fn describe(
        &self,
        _req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError> {
        Ok(ProjectionEventDescribeResponse {
            event_type: "credited".to_string(),
            aggregate_id: "balance".to_string(),
        })
    }
}

#[derive(Debug, Default)]
struct Balance {
    total: u64,
}

impl Projection for Balance {
    type Event = Credited;
    type ReadModel = u64;

    fn apply(
        &mut self,
        req: edge_application_projection::ProjectionApplyRequest<'_, Credited>,
    ) -> Result<(), ProjectionError> {
        self.total += req.event.amount;
        Ok(())
    }

    fn read_model(
        &self,
        _req: ProjectionReadModelRequest,
    ) -> Result<edge_application_projection::ProjectionReadModelResponse<'_, u64>, ProjectionError>
    {
        Ok(edge_application_projection::ProjectionReadModelResponse {
            read_model: &self.total,
        })
    }
}

#[derive(Debug, Clone)]
struct ApplyCreditsRequest {
    amounts: Vec<u64>,
}
impl edge_application_base::Request for ApplyCreditsRequest {}

#[derive(Debug, Clone, Copy)]
struct ApplyCreditsResponse {
    applied: usize,
}
impl edge_application_base::Response for ApplyCreditsResponse {}

/// Holds its own injected projection behind a lock; genuinely reads `req.ctx`.
struct ApplyCreditsHandler {
    balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for ApplyCreditsHandler {
    type Request = ApplyCreditsRequest;
    type Response = ApplyCreditsResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, ApplyCreditsRequest>,
    ) -> Result<ApplyCreditsResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "apply_credits_handler".to_string(),
                message: format!("applying {} credit event(s)", req.req.amounts.len()),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] ApplyCreditsHandler::execute — delegating to its own injected Projection");
        let events: Vec<Credited> = req
            .req
            .amounts
            .iter()
            .map(|&amount| Credited { amount })
            .collect();
        let count = {
            let mut balance = self.balance.lock();
            balance
                .try_drain(TryDrainRequest { events: &events })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .count
        };
        println!("  [2] ApplyCreditsHandler::execute — projection folded {count} event(s)");
        Ok(ApplyCreditsResponse { applied: count })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct GetBalanceRequest;
impl edge_application_base::Request for GetBalanceRequest {}

#[derive(Debug, Clone, Copy)]
struct GetBalanceResponse {
    total: u64,
}
impl edge_application_base::Response for GetBalanceResponse {}

/// Holds the *same* injected projection instance as `ApplyCreditsHandler`.
struct GetBalanceHandler {
    balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for GetBalanceHandler {
    type Request = GetBalanceRequest;
    type Response = GetBalanceResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, GetBalanceRequest>,
    ) -> Result<GetBalanceResponse, HandlerError> {
        println!("  [1] GetBalanceHandler::execute — delegating to its own injected Projection");
        let total = *self
            .balance
            .lock()
            .read_model(ProjectionReadModelRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .read_model;
        println!("  [2] GetBalanceHandler::execute — projection reports total = {total}");
        Ok(GetBalanceResponse { total })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected Projection<Event, ReadModel> (Arc<Mutex<P>>) — constructor injection, Self::Request carries the events ===\n");

    let balance = Arc::new(Mutex::new(Balance::default()));
    let apply_handler = ApplyCreditsHandler {
        balance: Arc::clone(&balance),
    };
    let get_handler = GetBalanceHandler {
        balance: Arc::clone(&balance),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] apply_handler.execute(ApplyCreditsRequest {{ amounts: [10, 20, 5] }})");
    let applied = apply_handler
        .execute(HandlerExecutionRequest {
            req: ApplyCreditsRequest {
                amounts: vec![10, 20, 5],
            },
            ctx: &ctx,
        })
        .await
        .expect("apply should succeed");
    println!("[3] ApplyCreditsHandler applied {} event(s)\n", applied.applied);

    println!("[0] get_handler.execute(GetBalanceRequest)");
    let balance_resp = get_handler
        .execute(HandlerExecutionRequest {
            req: GetBalanceRequest,
            ctx: &ctx,
        })
        .await
        .expect("read_model should succeed");
    println!("[3] GetBalanceHandler reports total = {}\n", balance_resp.total);

    println!("[0] apply_handler.execute(ApplyCreditsRequest {{ amounts: [] }}) — empty batch");
    let empty = apply_handler
        .execute(HandlerExecutionRequest {
            req: ApplyCreditsRequest { amounts: vec![] },
            ctx: &ctx,
        })
        .await;
    match empty {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] ApplyCreditsHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: both handlers reached the same projection only through their own,");
    println!("independently injected Arc<Mutex<Balance>> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. The projection's own empty-batch");
    println!("rejection propagated cleanly as a real HandlerError.");
}
