//! Integration tests for `SagaStore` and the `new_in_memory_saga_store`
//! SAF factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Command, Domain, DomainEvent, Saga, SagaError, SagaStore};

#[derive(Clone)]
struct Tick {
    id: String,
}
impl DomainEvent for Tick {
    fn aggregate_id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone)]
struct Advance;
impl Command for Advance {
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct CounterSaga {
    ticks: u32,
}
impl Saga for CounterSaga {
    type SagaId = String;
    type Event = Tick;
    type Command = Advance;
    fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
        self.ticks += 1;
        vec![Advance]
    }
    fn is_complete(&self) -> bool {
        self.ticks >= 3
    }
}

fn store() -> Box<dyn SagaStore<SagaInstance = CounterSaga>> {
    Domain::new_in_memory_saga_store::<CounterSaga>()
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_register_and_get_round_trip_happy() {
    let mut s = store();
    s.register("s1".to_string(), CounterSaga::default())
        .unwrap();
    assert!(s.get(&"s1".to_string()).is_ok());
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_lookup_on_empty_returns_not_found_error() {
    let s = store();
    assert_eq!(
        s.get(&"absent".to_string()).unwrap_err(),
        SagaError::NotFound("absent".to_string())
    );
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_starts_empty_edge() {
    let s = store();
    assert!(s.get(&"anything".to_string()).is_err());
}

/// @covers: SagaStore::register
#[test]
fn test_register_new_id_returns_ok_happy() {
    let mut s = store();
    assert!(s
        .register("order-1".to_string(), CounterSaga::default())
        .is_ok());
}

/// @covers: SagaStore::register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut s = store();
    s.register("order-1".to_string(), CounterSaga::default())
        .unwrap();
    assert_eq!(
        s.register("order-1".to_string(), CounterSaga::default())
            .unwrap_err(),
        SagaError::AlreadyRegistered("order-1".to_string())
    );
}

/// @covers: SagaStore::register
#[test]
fn test_register_distinct_ids_are_independent_edge() {
    let mut s = store();
    s.register("a".to_string(), CounterSaga::default())
        .unwrap();
    s.register("b".to_string(), CounterSaga::default())
        .unwrap();
    assert!(s.get(&"a".to_string()).is_ok());
    assert!(s.get(&"b".to_string()).is_ok());
}

/// @covers: SagaStore::get
#[test]
fn test_get_registered_id_returns_saga_happy() {
    let mut s = store();
    s.register("s1".to_string(), CounterSaga::default())
        .unwrap();
    let saga = s.get(&"s1".to_string()).unwrap();
    assert!(!saga.is_complete());
}

/// @covers: SagaStore::get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let s = store();
    assert_eq!(
        s.get(&"ghost".to_string()).unwrap_err(),
        SagaError::NotFound("ghost".to_string())
    );
}

/// @covers: SagaStore::get
#[test]
fn test_get_after_registering_other_id_still_not_found_edge() {
    let mut s = store();
    s.register("present".to_string(), CounterSaga::default())
        .unwrap();
    assert!(s.get(&"missing".to_string()).is_err());
}
