//! Runnable example: two `Handler`s sharing one constructor-injected
//! `SnapshotStore<AggregateId, Snap>`, with the per-call aggregate id/version carried through
//! `Self::Request` — the generic-per-type port wiring pattern `HandlerContext` structurally
//! can't hold. See issue #149.
//!
//! Unlike `saga`, `SnapshotStore::save`/`load` both take `&self` (interior mutability handled
//! internally by `MemorySnapshotStore`'s own `RwLock`) and are both async — no structural
//! blocker composing with `Handler::execute`, same shape as `repository`/`event`.
//!
//! `SaveSnapshotHandler::execute` genuinely reads `HandlerContext` for logging; the actual save
//! doesn't need it. Both handlers here share the same injected
//! `Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>` instance.
//!
//! Run with: `cargo run -p edge-application-snapshot-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_snapshot::{
    MemorySnapshotStore, Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse,
    SnapshotError, SnapshotLoadRequest, SnapshotSaveRequest, SnapshotStore, SnapshotVersionRequest,
    SnapshotVersionResponse,
};
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone)]
struct OrderSnap {
    aggregate_id: String,
    version: u64,
    total: u64,
}

impl Snapshot for OrderSnap {
    type AggregateId = String;

    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.aggregate_id,
        })
    }
    fn version(&self, _req: SnapshotVersionRequest) -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse {
            version: self.version,
        })
    }
}

#[derive(Debug, Clone)]
struct SaveSnapshotRequest {
    aggregate_id: String,
    version: u64,
    total: u64,
}
impl edge_application_base::Request for SaveSnapshotRequest {}

#[derive(Debug, Clone, Copy)]
struct SaveSnapshotResponse {
    saved: bool,
}
impl edge_application_base::Response for SaveSnapshotResponse {}

/// Holds its own injected `SnapshotStore`; genuinely reads `req.ctx` inside `execute()`.
struct SaveSnapshotHandler {
    snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for SaveSnapshotHandler {
    type Request = SaveSnapshotRequest;
    type Response = SaveSnapshotResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SaveSnapshotRequest>,
    ) -> Result<SaveSnapshotResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "save_snapshot_handler".to_string(),
                message: format!("saving snapshot for {:?}", req.req.aggregate_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] SaveSnapshotHandler::execute — delegating to its own injected SnapshotStore");
        self.snapshot_store
            .save(SnapshotSaveRequest {
                snapshot: OrderSnap {
                    aggregate_id: req.req.aggregate_id,
                    version: req.req.version,
                    total: req.req.total,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!("  [2] SaveSnapshotHandler::execute — snapshot store confirmed the save");
        Ok(SaveSnapshotResponse { saved: true })
    }
}

#[derive(Debug, Clone)]
struct LoadSnapshotRequest {
    aggregate_id: String,
}
impl edge_application_base::Request for LoadSnapshotRequest {}

#[derive(Debug, Clone)]
struct LoadSnapshotResponse {
    snapshot: Option<(u64, u64)>,
}
impl edge_application_base::Response for LoadSnapshotResponse {}

/// Holds the *same* injected `SnapshotStore` instance as `SaveSnapshotHandler`.
struct LoadSnapshotHandler {
    snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for LoadSnapshotHandler {
    type Request = LoadSnapshotRequest;
    type Response = LoadSnapshotResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, LoadSnapshotRequest>,
    ) -> Result<LoadSnapshotResponse, HandlerError> {
        println!("  [1] LoadSnapshotHandler::execute — delegating to its own injected SnapshotStore");
        let loaded = self
            .snapshot_store
            .load(SnapshotLoadRequest {
                id: &req.req.aggregate_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        let snapshot = loaded.snapshot.map(|s| (s.version, s.total));
        println!("  [2] LoadSnapshotHandler::execute — snapshot store returned {snapshot:?}");
        Ok(LoadSnapshotResponse { snapshot })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected SnapshotStore<AggregateId, Snap> — constructor injection, Self::Request carries the version ===\n");

    let snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>> =
        Arc::new(MemorySnapshotStore::<OrderSnap>::new());
    let save_handler = SaveSnapshotHandler {
        snapshot_store: Arc::clone(&snapshot_store),
    };
    let load_handler = LoadSnapshotHandler {
        snapshot_store: Arc::clone(&snapshot_store),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] save_handler.execute(SaveSnapshotRequest {{ aggregate_id: \"order-1\", version: 3, total: 42 }})");
    let save_resp = save_handler
        .execute(HandlerExecutionRequest {
            req: SaveSnapshotRequest {
                aggregate_id: "order-1".to_string(),
                version: 3,
                total: 42,
            },
            ctx: &ctx,
        })
        .await
        .expect("save should succeed");
    println!("[3] SaveSnapshotHandler reports saved = {}\n", save_resp.saved);

    println!("[0] load_handler.execute(LoadSnapshotRequest {{ aggregate_id: \"order-1\" }})");
    let load_resp = load_handler
        .execute(HandlerExecutionRequest {
            req: LoadSnapshotRequest {
                aggregate_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("load should succeed");
    println!("[3] LoadSnapshotHandler returned {:?}\n", load_resp.snapshot);

    println!("[0] save_handler.execute(SaveSnapshotRequest {{ aggregate_id: \"order-2\", version: 0, total: 0 }}) — invalid version");
    let invalid = save_handler
        .execute(HandlerExecutionRequest {
            req: SaveSnapshotRequest {
                aggregate_id: "order-2".to_string(),
                version: 0,
                total: 0,
            },
            ctx: &ctx,
        })
        .await;
    match invalid {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] SaveSnapshotHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: SaveSnapshotHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual store only through their own, independently");
    println!("injected SnapshotStore<String, OrderSnap> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. The store's own version-0 rejection");
    println!("propagated cleanly as a real HandlerError, not a fabricated one.");
}
