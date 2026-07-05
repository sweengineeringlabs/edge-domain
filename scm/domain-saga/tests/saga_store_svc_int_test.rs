//! SAF tests — `SagaStore` trait via `InMemorySagaStore`.
// @allow: no_mocks_in_integration

use edge_domain_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_domain_saga::{Command, CommandError, DomainEvent, InMemorySagaStore, Saga, SagaError, SagaStore};
use futures::future::BoxFuture;

#[derive(Clone)]
struct RegEvt;

impl DomainEvent for RegEvt {
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "reg-test" })
    }
}

#[derive(Clone, Debug)]
struct RegCmd;

impl Command for RegCmd {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct PingSaga {
    done: bool,
}

impl Saga for PingSaga {
    type SagaId = String;
    type Event = RegEvt;
    type Command = RegCmd;

    fn handle(&mut self, _e: &Self::Event) -> Vec<Self::Command> {
        self.done = true;
        vec![]
    }

    fn is_complete(&self) -> bool {
        self.done
    }
}

fn new_store() -> InMemorySagaStore<PingSaga> {
    InMemorySagaStore::new()
}

/// @covers: register
#[test]
fn test_register_new_saga_returns_ok_happy() {
    let mut store = new_store();
    assert_eq!(store.register("s1".to_string(), PingSaga::default()), Ok(()));
}

/// @covers: register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut store = new_store();
    store.register("s1".to_string(), PingSaga::default()).ok();
    let err = match store.register("s1".to_string(), PingSaga::default()) {
        Err(e) => e,
        Ok(()) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
}

/// @covers: register
#[test]
fn test_register_multiple_sagas_with_different_ids_edge() {
    let mut store = new_store();
    assert_eq!(store.register("a".to_string(), PingSaga::default()), Ok(()));
    assert_eq!(store.register("b".to_string(), PingSaga::default()), Ok(()));
    assert!(store.get(&"a".to_string()).is_ok());
    assert!(store.get(&"b".to_string()).is_ok());
}

/// @covers: get
#[test]
fn test_get_registered_saga_returns_saga_happy() {
    let mut store = new_store();
    store.register("s1".to_string(), PingSaga::default()).ok();
    let saga = store.get(&"s1".to_string());
    assert_eq!(saga.is_ok(), true);
    if let Ok(s) = saga {
        assert!(!s.is_complete());
    }
}

/// @covers: get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let store = new_store();
    let err = match store.get(&"ghost".to_string()) {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::NotFound("ghost".to_string()));
}

/// @covers: get
#[test]
fn test_get_after_register_is_consistent_edge() {
    let mut store = new_store();
    store.register("id1".to_string(), PingSaga::default()).ok();
    let saga = match store.get(&"id1".to_string()) {
        Ok(s) => s,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert!(!saga.is_complete());
}
