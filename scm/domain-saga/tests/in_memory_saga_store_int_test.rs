//! Integration tests for `InMemorySagaStore`.
// @allow: no_mocks_in_integration

use edge_domain_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_domain_saga::{Command, CommandError, DomainEvent, InMemorySagaStore, Saga, SagaStore};
use futures::future::BoxFuture;

#[derive(Clone)]
struct RegEvt2;

impl DomainEvent for RegEvt2 {
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "reg2" })
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
fn test_new_store_is_empty_happy() {
    let store = InMemorySagaStore::<CountingSaga>::new();
    assert!(store.get(&"x".to_string()).is_err());
}

#[test]
fn test_default_store_is_empty_error() {
    let store = InMemorySagaStore::<CountingSaga>::default();
    assert!(store.get(&"x".to_string()).is_err());
}

#[test]
fn test_store_stores_and_retrieves_saga_edge() {
    let mut store = InMemorySagaStore::<CountingSaga>::new();
    store.register("c1".to_string(), CountingSaga::default()).ok();
    let saga = match store.get(&"c1".to_string()) {
        Ok(s) => s,
        Err(e) => panic!("expected saga, got: {e}"),
    };
    assert!(!saga.is_complete());
}
