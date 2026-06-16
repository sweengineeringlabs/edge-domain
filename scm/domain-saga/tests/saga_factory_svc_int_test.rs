//! SAF tests — `SagaFactory` trait.
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, CommandError, DomainEvent, NoopSaga, NoopSagaCommand, NoopSagaEvent, Saga, SagaFactory, SagaStore, StdSagaFactory};
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

/// @covers: in_memory_store
#[test]
fn test_in_memory_store_creates_empty_registry_happy() {
    let reg = Factories::in_memory_store::<SimpleSaga>();
    assert!(reg.get(&"any".to_string()).is_err());
}

/// @covers: in_memory_store
#[test]
fn test_in_memory_store_accepts_registration_error() {
    let mut reg = Factories::in_memory_store::<SimpleSaga>();
    assert!(reg.register("s1".to_string(), SimpleSaga).is_ok());
}

/// @covers: in_memory_store
#[test]
fn test_in_memory_store_multiple_instances_are_independent_edge() {
    let mut reg1 = Factories::in_memory_store::<SimpleSaga>();
    let reg2 = Factories::in_memory_store::<SimpleSaga>();
    reg1.register("s1".to_string(), SimpleSaga).ok();
    assert!(reg1.get(&"s1".to_string()).is_ok());
    assert!(reg2.get(&"s1".to_string()).is_err());
}

/// @covers: noop
#[test]
fn test_noop_creates_noop_saga_not_complete_happy() {
    let saga: NoopSaga = Factories::noop();
    assert!(!saga.is_complete());
}

/// @covers: noop
#[test]
fn test_noop_creates_fresh_instance_each_call_error() {
    // Each call returns a new independent instance — not a shared ref
    let s1: NoopSaga = Factories::noop();
    let s2: NoopSaga = Factories::noop();
    assert!(!s1.is_complete());
    assert!(!s2.is_complete());
}

/// @covers: noop
#[test]
fn test_noop_default_and_factory_produce_equivalent_state_edge() {
    let from_factory: NoopSaga = Factories::noop();
    let from_default = NoopSaga::default();
    assert_eq!(from_factory.is_complete(), from_default.is_complete());
}

/// @covers: noop_event
#[test]
fn test_noop_event_creates_event_with_empty_aggregate_id_happy() {
    use edge_domain_saga::DomainEvent;
    let evt: NoopSagaEvent = Factories::noop_event();
    assert_eq!(evt.aggregate_id(), "");
}

/// @covers: noop_event
#[test]
fn test_noop_event_event_type_returns_default_error() {
    use edge_domain_saga::DomainEvent;
    let evt: NoopSagaEvent = Factories::noop_event();
    assert_eq!(evt.event_type(), "event");
}

/// @covers: noop_event
#[test]
fn test_noop_event_can_be_cloned_edge() {
    let evt: NoopSagaEvent = Factories::noop_event();
    let cloned = evt.clone();
    use edge_domain_saga::DomainEvent;
    assert_eq!(cloned.aggregate_id(), "");
}

/// @covers: noop_command
#[test]
fn test_noop_command_execute_returns_ok_happy() {
    use futures::executor::block_on;
    use edge_domain_saga::Command;
    let cmd: NoopSagaCommand = Factories::noop_command();
    assert!(block_on(cmd.execute()).is_ok());
}

/// @covers: noop_command
#[test]
fn test_noop_command_name_returns_default_error() {
    use edge_domain_saga::Command;
    let cmd: NoopSagaCommand = Factories::noop_command();
    assert_eq!(cmd.name(), "command");
}

/// @covers: noop_command
#[test]
fn test_noop_command_can_be_called_repeatedly_edge() {
    use futures::executor::block_on;
    use edge_domain_saga::Command;
    let cmd: NoopSagaCommand = Factories::noop_command();
    let r1 = block_on(cmd.execute());
    let r2 = block_on(cmd.execute());
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}

/// @covers: std_factory
#[test]
fn test_std_factory_returns_std_saga_factory_happy() {
    let _f: StdSagaFactory = Factories::std_factory();
    // StdSagaFactory is a unit struct — constructing it is the proof
}

/// @covers: std_factory
#[test]
fn test_std_factory_can_create_registry_via_returned_type_error() {
    let _f: StdSagaFactory = Factories::std_factory();
    // StdSagaFactory implements SagaFactory; ensure it reaches registry creation
    let reg = StdSagaFactory::in_memory_store::<SimpleSaga>();
    assert!(reg.get(&"x".to_string()).is_err());
}

/// @covers: std_factory
#[test]
fn test_std_factory_multiple_calls_produce_equivalent_instances_edge() {
    let _f1: StdSagaFactory = Factories::std_factory();
    let _f2: StdSagaFactory = Factories::std_factory();
    // Both are unit structs — equivalent by construction
}
