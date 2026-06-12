//! SAF tests — `SagaFactory` trait.
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, Saga, SagaFactory, SagaRegistry};
use futures::future::BoxFuture;

#[derive(Clone)]
struct FactEvt;

impl DomainEvent for FactEvt {
    fn aggregate_id(&self) -> &str {
        "fact-test"
    }
}

#[derive(Clone, Debug)]
struct FactCmd;

impl Command for FactCmd {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct SimpleSaga;

impl Saga for SimpleSaga {
    type SagaId = String;
    type Event = FactEvt;
    type Command = FactCmd;

    fn handle(&mut self, _e: &Self::Event) -> Vec<Self::Command> {
        vec![]
    }

    fn is_complete(&self) -> bool {
        true
    }
}

struct Factories;
impl SagaFactory for Factories {}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_creates_empty_registry_happy() {
    let reg = Factories::in_memory_registry::<SimpleSaga>();
    assert!(reg.get(&"any".to_string()).is_err());
}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_accepts_registration_error() {
    let mut reg = Factories::in_memory_registry::<SimpleSaga>();
    assert!(reg.register("s1".to_string(), SimpleSaga).is_ok());
}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_multiple_instances_are_independent_edge() {
    let mut reg1 = Factories::in_memory_registry::<SimpleSaga>();
    let reg2 = Factories::in_memory_registry::<SimpleSaga>();
    reg1.register("s1".to_string(), SimpleSaga).ok();
    assert!(reg1.get(&"s1".to_string()).is_ok());
    assert!(reg2.get(&"s1".to_string()).is_err());
}
