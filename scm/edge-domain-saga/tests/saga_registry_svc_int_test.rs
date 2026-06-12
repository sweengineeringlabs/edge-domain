//! SAF tests — `SagaRegistry` trait via `InMemorySagaRegistry`.
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, InMemorySagaRegistry, Saga, SagaError, SagaRegistry};
use futures::future::BoxFuture;

#[derive(Clone)]
struct RegEvt;

impl DomainEvent for RegEvt {
    fn aggregate_id(&self) -> &str {
        "reg-test"
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

fn new_reg() -> InMemorySagaRegistry<PingSaga> {
    InMemorySagaRegistry::new()
}

/// @covers: register
#[test]
fn test_register_new_saga_returns_ok_happy() {
    let mut reg = new_reg();
    assert!(reg.register("s1".to_string(), PingSaga::default()).is_ok());
}

/// @covers: register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut reg = new_reg();
    reg.register("s1".to_string(), PingSaga::default()).ok();
    let err = match reg.register("s1".to_string(), PingSaga::default()) {
        Err(e) => e,
        Ok(()) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
}

/// @covers: register
#[test]
fn test_register_multiple_sagas_with_different_ids_edge() {
    let mut reg = new_reg();
    assert!(reg.register("a".to_string(), PingSaga::default()).is_ok());
    assert!(reg.register("b".to_string(), PingSaga::default()).is_ok());
    assert!(reg.get(&"a".to_string()).is_ok());
    assert!(reg.get(&"b".to_string()).is_ok());
}

/// @covers: get
#[test]
fn test_get_registered_saga_returns_saga_happy() {
    let mut reg = new_reg();
    reg.register("s1".to_string(), PingSaga::default()).ok();
    assert!(reg.get(&"s1".to_string()).is_ok());
}

/// @covers: get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let reg = new_reg();
    let err = match reg.get(&"ghost".to_string()) {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::NotFound("ghost".to_string()));
}

/// @covers: get
#[test]
fn test_get_after_register_is_consistent_edge() {
    let mut reg = new_reg();
    reg.register("id1".to_string(), PingSaga::default()).ok();
    let saga = match reg.get(&"id1".to_string()) {
        Ok(s) => s,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert!(!saga.is_complete());
}
