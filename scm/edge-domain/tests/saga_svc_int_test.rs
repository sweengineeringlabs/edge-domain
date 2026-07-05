//! SAF facade integration tests — the `Saga` trait is exported from the crate
//! root and implementable by downstream consumers.
//!
//! The fixture models an order-fulfillment saga with a compensation path so the
//! `_error` scenarios exercise a real rollback, not a contrived assertion.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Command, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventTypeRequest, EventTypeResponse, Saga,
};

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
    fn name(&self) -> &str {
        match self {
            OrderCommand::ReserveStock { .. } => "reserve-stock",
            OrderCommand::RefundCustomer { .. } => "refund-customer",
        }
    }
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
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

    fn handle(&mut self, event: &Self::Event) -> Vec<Self::Command> {
        match event {
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
        }
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
}

/// @covers: Saga::handle
#[test]
fn test_handle_order_placed_stages_reserve_stock_happy() {
    let mut saga = OrderSaga::default();
    let cmds = saga.handle(&OrderEvent::Placed {
        order_id: "order-1".to_string(),
    });
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
    let cmds = saga.handle(&OrderEvent::Confirmed {
        order_id: "order-1".to_string(),
    });
    assert!(cmds.is_empty());
}

/// @covers: Saga::handle
#[test]
fn test_handle_payment_failed_stages_compensating_refund_error() {
    let mut saga = OrderSaga::default();
    let cmds = saga.handle(&OrderEvent::PaymentFailed {
        order_id: "order-1".to_string(),
    });
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
    saga.handle(&OrderEvent::Confirmed {
        order_id: "order-1".to_string(),
    });
    assert!(saga.is_complete());
}

/// @covers: Saga::is_complete
#[test]
fn test_is_complete_before_any_event_returns_false_edge() {
    let saga = OrderSaga::default();
    assert!(!saga.is_complete());
}

/// @covers: Saga::is_complete
#[test]
fn test_is_complete_after_compensation_returns_true_error() {
    let mut saga = OrderSaga::default();
    saga.handle(&OrderEvent::PaymentFailed {
        order_id: "order-1".to_string(),
    });
    assert!(saga.is_complete());
}
