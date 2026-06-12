//! Integration tests for `InMemorySagaRegistry`.
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, InMemorySagaRegistry, Saga, SagaRegistry};
use futures::future::BoxFuture;

#[derive(Clone)]
struct RegEvt2;

impl DomainEvent for RegEvt2 {
    fn aggregate_id(&self) -> &str {
        "reg2"
    }
}

#[derive(Clone, Debug)]
struct RegCmd2;

impl Command for RegCmd2 {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct CountingSaga {
    count: u32,
}

impl Saga for CountingSaga {
    type SagaId = String;
    type Event = RegEvt2;
    type Command = RegCmd2;

    fn handle(&mut self, _e: &Self::Event) -> Vec<Self::Command> {
        self.count += 1;
        vec![]
    }

    fn is_complete(&self) -> bool {
        self.count >= 3
    }
}

#[test]
fn test_new_registry_is_empty_happy() {
    let reg = InMemorySagaRegistry::<CountingSaga>::new();
    assert!(reg.get(&"x".to_string()).is_err());
}

#[test]
fn test_default_registry_is_empty_error() {
    let reg = InMemorySagaRegistry::<CountingSaga>::default();
    assert!(reg.get(&"x".to_string()).is_err());
}

#[test]
fn test_registry_stores_and_retrieves_saga_edge() {
    let mut reg = InMemorySagaRegistry::<CountingSaga>::new();
    reg.register("c1".to_string(), CountingSaga::default()).ok();
    let saga = match reg.get(&"c1".to_string()) {
        Ok(s) => s,
        Err(e) => panic!("expected saga, got: {e}"),
    };
    assert!(!saga.is_complete());
}
