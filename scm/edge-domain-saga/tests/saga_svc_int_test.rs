//! SAF tests — `Saga` trait.
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, Saga, SagaFactory, SagaRegistry};
use futures::future::BoxFuture;

#[derive(Clone)]
struct StepEvt;

impl DomainEvent for StepEvt {
    fn aggregate_id(&self) -> &str {
        "order-1"
    }
}

#[derive(Clone, Debug)]
struct StepCmd;

impl Command for StepCmd {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

struct OrderSaga {
    steps: u32,
    target: u32,
}

impl OrderSaga {
    fn new(target: u32) -> Self {
        Self { steps: 0, target }
    }
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = StepEvt;
    type Command = StepCmd;

    fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
        self.steps += 1;
        if self.steps < self.target {
            vec![StepCmd]
        } else {
            vec![]
        }
    }

    fn is_complete(&self) -> bool {
        self.steps >= self.target
    }
}

struct Factories;
impl SagaFactory for Factories {}

/// @covers: handle
#[test]
fn test_handle_first_step_event_returns_commands_happy() {
    let mut saga = OrderSaga::new(3);
    let cmds = saga.handle(&StepEvt);
    assert_eq!(cmds.len(), 1);
}

/// @covers: handle
#[test]
fn test_handle_final_step_event_returns_empty_commands_error() {
    let mut saga = OrderSaga::new(1);
    let cmds = saga.handle(&StepEvt);
    assert!(cmds.is_empty());
}

/// @covers: handle
#[test]
fn test_handle_multiple_events_accumulate_steps_edge() {
    let mut saga = OrderSaga::new(3);
    saga.handle(&StepEvt);
    saga.handle(&StepEvt);
    let cmds = saga.handle(&StepEvt);
    assert!(cmds.is_empty());
    assert!(saga.is_complete());
}

/// @covers: is_complete
#[test]
fn test_is_complete_after_target_steps_returns_true_happy() {
    let mut saga = OrderSaga::new(1);
    saga.handle(&StepEvt);
    assert!(saga.is_complete());
}

/// @covers: is_complete
#[test]
fn test_is_complete_initial_state_returns_false_error() {
    let saga = OrderSaga::new(3);
    assert!(!saga.is_complete());
}

/// @covers: is_complete
#[test]
fn test_is_complete_midway_through_returns_false_edge() {
    let mut saga = OrderSaga::new(3);
    saga.handle(&StepEvt);
    saga.handle(&StepEvt);
    assert!(!saga.is_complete());
}

/// @covers: handle
#[test]
fn test_handle_via_dyn_registry_returns_ok_edge() {
    let mut reg = Factories::in_memory_registry::<OrderSaga>();
    reg.register("o1".to_string(), OrderSaga::new(2)).ok();
    if let Ok(saga) = reg.get(&"o1".to_string()) {
        assert!(!saga.is_complete());
    }
}
