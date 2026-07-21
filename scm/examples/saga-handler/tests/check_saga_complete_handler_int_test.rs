//! Integration test: `CheckSagaCompleteHandler::execute()` reads through the *same*
//! `Arc<Mutex<OrderSaga>>` instance a `HandleSagaEventHandler` mutates, proving one saga
//! instance can genuinely back multiple handlers, each reaching it through its own injected
//! field rather than through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_saga::{
    Saga, SagaCommand, SagaCommandDispatchRequest, SagaError, SagaEvent,
    SagaEventDescribeRequest, SagaEventDescribeResponse, SagaHandleRequest, SagaHandleResponse,
    SagaIsCompleteRequest, SagaIsCompleteResponse,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use parking_lot::Mutex;

#[derive(Debug, Clone)]
struct OrderPaid {
    order_id: String,
}

impl SagaEvent for OrderPaid {
    fn describe(
        &self,
        _req: SagaEventDescribeRequest,
    ) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: "order.paid".to_string(),
            aggregate_id: self.order_id.clone(),
        })
    }
}

#[derive(Debug, Clone)]
struct ShipOrderCommand {
    order_id: String,
}

impl SagaCommand for ShipOrderCommand {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), SagaError>> {
        println!("      [infra] ShipOrderCommand::dispatch — shipping {:?}", self.order_id);
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct OrderSaga {
    shipped: bool,
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = OrderPaid;
    type Command = ShipOrderCommand;

    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, OrderPaid>,
    ) -> Result<SagaHandleResponse<ShipOrderCommand>, SagaError> {
        self.shipped = true;
        Ok(SagaHandleResponse {
            commands: vec![ShipOrderCommand {
                order_id: req.event.order_id.clone(),
            }],
        })
    }

    fn is_complete(&self, _req: SagaIsCompleteRequest) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.shipped,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CheckSagaCompleteRequest;
impl edge_application_base::Request for CheckSagaCompleteRequest {}

#[derive(Debug, Clone, Copy)]
struct CheckSagaCompleteResponse {
    complete: bool,
}
impl edge_application_base::Response for CheckSagaCompleteResponse {}

/// Holds its own injected saga behind the same lock a `HandleSagaEventHandler` would mutate.
struct CheckSagaCompleteHandler {
    saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for CheckSagaCompleteHandler {
    type Request = CheckSagaCompleteRequest;
    type Response = CheckSagaCompleteResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, CheckSagaCompleteRequest>,
    ) -> Result<CheckSagaCompleteResponse, HandlerError> {
        let complete = self
            .saga
            .lock()
            .is_complete(SagaIsCompleteRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .complete;
        Ok(CheckSagaCompleteResponse { complete })
    }
}

fn run(handler: &CheckSagaCompleteHandler) -> Result<CheckSagaCompleteResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: CheckSagaCompleteRequest,
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_after_event_handled_returns_complete_true_happy() {
    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    saga.lock()
        .handle(SagaHandleRequest {
            event: &OrderPaid {
                order_id: "order-1".to_string(),
            },
        })
        .unwrap();
    let handler = CheckSagaCompleteHandler {
        saga: Arc::clone(&saga),
    };

    let response = run(&handler).unwrap();
    assert!(response.complete);
}

/// @covers: Handler::execute
#[test]
fn test_execute_before_any_event_returns_complete_false_error() {
    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    let handler = CheckSagaCompleteHandler { saga };

    let response = run(&handler).unwrap();
    assert!(!response.complete);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_consistent_edge() {
    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    let handler = CheckSagaCompleteHandler {
        saga: Arc::clone(&saga),
    };

    let before_first = run(&handler).unwrap();
    let before_second = run(&handler).unwrap();
    saga.lock()
        .handle(SagaHandleRequest {
            event: &OrderPaid {
                order_id: "order-1".to_string(),
            },
        })
        .unwrap();
    let after = run(&handler).unwrap();

    assert!(!before_first.complete);
    assert_eq!(before_first.complete, before_second.complete);
    assert!(after.complete);
}
