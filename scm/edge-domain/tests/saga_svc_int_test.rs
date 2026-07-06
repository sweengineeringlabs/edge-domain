//! SAF facade integration tests — the `Saga` trait is exported from the crate
//! root and implementable by downstream consumers.
//!
//! The fixture models an order-fulfillment saga with a compensation path so the
//! `_error` scenarios exercise a real rollback, not a contrived assertion.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Command, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventTypeRequest, EventTypeResponse, Saga, SagaError, SagaHandleRequest, SagaHandleResponse,
    SagaIsCompleteRequest, SagaIsCompleteResponse,
};
use edge_domain_command::{ExecutionRequest, NameRequest, NameResponse};

/// Events the saga reacts to.
#[derive(Clone)]
enum OrderEvent {
    Placed { order_id: String },
    Confirmed { order_id: String },
    PaymentFailed { order_id: String },
}

impl DomainEvent for OrderEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: match self {
                OrderEvent::Placed { .. } => "order.placed",
                OrderEvent::Confirmed { .. } => "order.confirmed",
                OrderEvent::PaymentFailed { .. } => "order.payment_failed",
            },
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: match self {
                OrderEvent::Placed { order_id }
                | OrderEvent::Confirmed { order_id }
                | OrderEvent::PaymentFailed { order_id } => order_id,
            },
        })
    }
}

/// Commands the saga can stage.
#[derive(Debug, Clone, PartialEq, Eq)]
enum OrderCommand {
    ReserveStock { order_id: String },
    RefundCustomer { order_id: String },
}

impl Command for OrderCommand {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, edge_domain::CommandError> {
        Ok(NameResponse {
            name: match self {
                OrderCommand::ReserveStock { .. } => "reserve-stock",
                OrderCommand::RefundCustomer { .. } => "refund-customer",
            }
            .to_string(),
        })
    }
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct OrderSaga {
    complete: bool,
    compensated: bool,
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = OrderEvent;
    type Command = OrderCommand;

    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        let commands = match req.event {
            OrderEvent::Placed { order_id } => {
                vec![OrderCommand::ReserveStock {
                    order_id: order_id.clone(),
                }]
            }
            OrderEvent::Confirmed { .. } => {
                self.complete = true;
                vec![]
            }
            OrderEvent::PaymentFailed { order_id } => {
                self.complete = true;
                self.compensated = true;
                vec![OrderCommand::RefundCustomer {
                    order_id: order_id.clone(),
                }]
            }
        };
        Ok(SagaHandleResponse { commands })
    }

    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.complete,
        })
    }
}

/// @covers: Saga::handle
#[test]
fn test_handle_order_placed_stages_reserve_stock_happy() {
    let mut saga = OrderSaga::default();
    let event = OrderEvent::Placed {
        order_id: "order-1".to_string(),
    };
    let cmds = saga
        .handle(SagaHandleRequest { event: &event })
        .unwrap()
        .commands;
    assert_eq!(
        cmds,
        vec![OrderCommand::ReserveStock {
            order_id: "order-1".to_string()
        }]
    );
}

/// @covers: Saga::handle
#[test]
fn test_handle_order_confirmed_stages_no_commands_edge() {
    let mut saga = OrderSaga::default();
    let event = OrderEvent::Confirmed {
        order_id: "order-1".to_string(),
    };
    let cmds = saga
        .handle(SagaHandleRequest { event: &event })
        .unwrap()
        .commands;
    assert!(cmds.is_empty());
}

/// @covers: Saga::handle
#[test]
fn test_handle_payment_failed_stages_compensating_refund_error() {
    let mut saga = OrderSaga::default();
    let event = OrderEvent::PaymentFailed {
        order_id: "order-1".to_string(),
    };
    let cmds = saga
        .handle(SagaHandleRequest { event: &event })
        .unwrap()
        .commands;
    assert_eq!(
        cmds,
        vec![OrderCommand::RefundCustomer {
            order_id: "order-1".to_string()
        }]
    );
    assert!(saga.compensated);
}

/// @covers: Saga::is_complete
#[test]
fn test_is_complete_after_confirmation_returns_true_happy() {
    let mut saga = OrderSaga::default();
    let event = OrderEvent::Confirmed {
        order_id: "order-1".to_string(),
    };
    saga.handle(SagaHandleRequest { event: &event }).unwrap();
    assert!(saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: Saga::is_complete
#[test]
fn test_is_complete_before_any_event_returns_false_edge() {
    let saga = OrderSaga::default();
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: Saga::is_complete
#[test]
fn test_is_complete_after_compensation_returns_true_error() {
    let mut saga = OrderSaga::default();
    let event = OrderEvent::PaymentFailed {
        order_id: "order-1".to_string(),
    };
    saga.handle(SagaHandleRequest { event: &event }).unwrap();
    assert!(saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}
