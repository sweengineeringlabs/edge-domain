//! Integration tests for `SagaRegistry` and the `new_in_memory_saga_registry`
//! SAF factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Command, Domain, DomainEvent, Saga, SagaError, SagaRegistry};

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

fn registry() -> Box<dyn SagaRegistry<CounterSaga>> {
    Domain::new_in_memory_saga_registry::<CounterSaga>()
}

/// @covers: new_in_memory_saga_registry
#[test]
fn test_new_in_memory_saga_registry_register_and_get_round_trip_happy() {
    let mut reg = registry();
    reg.register("s1".to_string(), CounterSaga::default())
        .unwrap();
    assert!(reg.get(&"s1".to_string()).is_ok());
}

/// @covers: new_in_memory_saga_registry
#[test]
fn test_new_in_memory_saga_registry_lookup_on_empty_returns_not_found_error() {
    let reg = registry();
    assert_eq!(
        reg.get(&"absent".to_string()).unwrap_err(),
        SagaError::NotFound("absent".to_string())
    );
}

/// @covers: new_in_memory_saga_registry
#[test]
fn test_new_in_memory_saga_registry_starts_empty_edge() {
    let reg = registry();
    assert!(reg.get(&"anything".to_string()).is_err());
}

/// @covers: SagaRegistry::register
#[test]
fn test_register_new_id_returns_ok_happy() {
    let mut reg = registry();
    assert!(reg
        .register("order-1".to_string(), CounterSaga::default())
        .is_ok());
}

/// @covers: SagaRegistry::register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut reg = registry();
    reg.register("order-1".to_string(), CounterSaga::default())
        .unwrap();
    assert_eq!(
        reg.register("order-1".to_string(), CounterSaga::default())
            .unwrap_err(),
        SagaError::AlreadyRegistered("order-1".to_string())
    );
}

/// @covers: SagaRegistry::register
#[test]
fn test_register_distinct_ids_are_independent_edge() {
    let mut reg = registry();
    reg.register("a".to_string(), CounterSaga::default())
        .unwrap();
    reg.register("b".to_string(), CounterSaga::default())
        .unwrap();
    assert!(reg.get(&"a".to_string()).is_ok());
    assert!(reg.get(&"b".to_string()).is_ok());
}

/// @covers: SagaRegistry::get
#[test]
fn test_get_registered_id_returns_saga_happy() {
    let mut reg = registry();
    reg.register("s1".to_string(), CounterSaga::default())
        .unwrap();
    let saga = reg.get(&"s1".to_string()).unwrap();
    assert!(!saga.is_complete());
}

/// @covers: SagaRegistry::get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let reg = registry();
    assert_eq!(
        reg.get(&"ghost".to_string()).unwrap_err(),
        SagaError::NotFound("ghost".to_string())
    );
}

/// @covers: SagaRegistry::get
#[test]
fn test_get_after_registering_other_id_still_not_found_edge() {
    let mut reg = registry();
    reg.register("present".to_string(), CounterSaga::default())
        .unwrap();
    assert!(reg.get(&"missing".to_string()).is_err());
}
