//! Integration tests for [`StdSagaFactory`].
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, Saga, SagaFactory, SagaRegistry, StdSagaFactory};
use futures::future::BoxFuture;

#[derive(Clone)]
struct FactoryEvt;

impl DomainEvent for FactoryEvt {
    fn aggregate_id(&self) -> &str {
        "factory-test"
    }
}

#[derive(Clone, Debug)]
struct FactoryCmd;

impl Command for FactoryCmd {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct FactorySaga;

impl Saga for FactorySaga {
    type SagaId = String;
    type Event = FactoryEvt;
    type Command = FactoryCmd;

    fn handle(&mut self, _e: &Self::Event) -> Vec<Self::Command> {
        vec![]
    }

    fn is_complete(&self) -> bool {
        true
    }
}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_std_factory_creates_empty_registry_happy() {
    let reg = StdSagaFactory::in_memory_registry::<FactorySaga>();
    assert!(reg.get(&"any".to_string()).is_err());
}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_std_factory_accepts_registration_error() {
    let mut reg = StdSagaFactory::in_memory_registry::<FactorySaga>();
    assert!(reg.register("s1".to_string(), FactorySaga).is_ok());
}

/// @covers: in_memory_registry
#[test]
fn test_in_memory_registry_std_factory_multiple_instances_independent_edge() {
    let mut reg1 = StdSagaFactory::in_memory_registry::<FactorySaga>();
    let reg2 = StdSagaFactory::in_memory_registry::<FactorySaga>();
    reg1.register("s1".to_string(), FactorySaga).ok();
    assert!(reg1.get(&"s1".to_string()).is_ok());
    assert!(reg2.get(&"s1".to_string()).is_err());
}
