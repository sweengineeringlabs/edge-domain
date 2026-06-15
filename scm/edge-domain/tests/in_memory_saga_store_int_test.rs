//! Integration tests for the in-memory saga store implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Command, Domain, DomainEvent, Saga, SagaError, SagaStore};

#[derive(Clone)]
struct Pulse {
    id: String,
}
impl DomainEvent for Pulse {
    fn aggregate_id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone)]
struct Step;
impl Command for Step {
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct PulseSaga {
    steps: u32,
}
impl Saga for PulseSaga {
    type SagaId = String;
    type Event = Pulse;
    type Command = Step;
    fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
        self.steps += 1;
        vec![Step]
    }
    fn is_complete(&self) -> bool {
        self.steps >= 2
    }
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_stores_and_retrieves_saga() {
    let mut store: Box<dyn SagaStore<SagaInstance = PulseSaga>> =
        Domain::new_in_memory_saga_store::<PulseSaga>();
    store.register("p1".to_string(), PulseSaga::default())
        .unwrap();
    let saga = store.get(&"p1".to_string()).unwrap();
    assert!(!saga.is_complete());
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_rejects_duplicate_registration() {
    let mut store: Box<dyn SagaStore<SagaInstance = PulseSaga>> =
        Domain::new_in_memory_saga_store::<PulseSaga>();
    store.register("p1".to_string(), PulseSaga::default())
        .unwrap();
    assert_eq!(
        store.register("p1".to_string(), PulseSaga::default())
            .unwrap_err(),
        SagaError::AlreadyRegistered("p1".to_string())
    );
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_lookup_of_unknown_id_is_not_found() {
    let store: Box<dyn SagaStore<SagaInstance = PulseSaga>> = Domain::new_in_memory_saga_store::<PulseSaga>();
    assert_eq!(
        store.get(&"unknown".to_string()).unwrap_err(),
        SagaError::NotFound("unknown".to_string())
    );
}
