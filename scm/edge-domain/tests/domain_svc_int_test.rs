//! Integration tests for saf/edge_domain_svc public API.
//!
//! Covers all factory functions in edge_domain_svc.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::*;
use std::sync::Arc;

/// @covers: echo_handler
#[test]
fn test_echo_handler() {
    let _: Arc<dyn edge_domain::Handler<String, String>> = Domain::echo_handler("id", "/path");
}

/// @covers: echo_handler
#[tokio::test]
async fn test_echo_handler_returns_input_as_output() {
    let h = Domain::echo_handler::<String>("echo", "/echo");
    assert_eq!(h.execute("ping".into()).await.unwrap(), "ping");
}

/// @covers: new_handler_registry
#[test]
fn test_new_handler_registry_returns_empty_registry() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.is_empty());
}

/// @covers: new_service_registry
#[test]
fn test_new_service_registry_returns_empty_registry() {
    let reg = Domain::new_service_registry::<String, String>();
    assert!(reg.is_empty());
}

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository() {
    let _: Arc<dyn edge_domain::Repository<String, u32>> = Domain::new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository() {
    let _: Arc<dyn edge_domain::QueryableRepository<String, u32>> =
        Domain::new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_returns_functional_store() {
    use edge_domain::Spec;
    struct Any;
    impl Spec<String> for Any {
        fn matches(&self, _: &String) -> bool {
            true
        }
    }
    let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
    repo.save(1u32, "x".to_string()).await.unwrap();
    assert_eq!(repo.count_by(&Any).await.unwrap(), 1);
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_saves_and_finds_entity() {
    let repo = Domain::new_in_memory_repository::<String, u32>();
    repo.save(1u32, "x".to_string()).await.unwrap();
    assert!(repo.find(&1u32).await.unwrap().is_some());
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_supports_count_by() {
    use edge_domain::Spec;
    struct Any;
    impl Spec<String> for Any {
        fn matches(&self, _: &String) -> bool {
            true
        }
    }
    let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
    repo.save(1u32, "x".to_string()).await.unwrap();
    assert_eq!(repo.count_by(&Any).await.unwrap(), 1);
}

/// @covers: validate_config
#[test]
fn test_validate_config_returns_ok_for_valid_input() {
    use edge_domain::{Validator, ValidatorError};
    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), ValidatorError> {
            Ok(())
        }
    }
    assert!(Domain::validate_config(&AlwaysValid).is_ok());
}

/// @covers: validate_config
#[test]
fn test_validate_config_returns_err_for_invalid_input() {
    use edge_domain::{Validator, ValidatorError};
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self) -> Result<(), ValidatorError> {
            Err(ValidatorError::Invalid("bad".into()))
        }
    }
    assert!(Domain::validate_config(&AlwaysInvalid).is_err());
}

/// @covers: direct_command_bus
#[test]
fn test_direct_command_bus_returns_arc_command_bus() {
    let bus = Domain::direct_command_bus();
    let _: Arc<dyn edge_domain::CommandBus> = bus;
}

/// @covers: noop_event_publisher
#[test]
fn test_noop_event_publisher_returns_arc_event_publisher() {
    let pub_ = Domain::noop_event_publisher();
    let _: Arc<dyn edge_domain::EventPublisher> = pub_;
}

/// @covers: direct_query_bus
#[test]
fn test_direct_query_bus_returns_arc_query_bus() {
    let bus = Domain::direct_query_bus::<String>();
    let _: Arc<dyn edge_domain::QueryBus<String>> = bus;
}

#[derive(Clone)]
struct AnyEvent;
impl edge_domain::DomainEvent for AnyEvent {
    fn event_type(&self) -> &str {
        "test.any"
    }
    fn aggregate_id(&self) -> &str {
        "id"
    }
    fn occurred_at(&self) -> std::time::SystemTime {
        std::time::SystemTime::now()
    }
}

/// @covers: new_in_memory_event_store
#[test]
fn test_new_in_memory_event_store_returns_arc_event_store() {
    let _: Arc<dyn edge_domain::EventStore<AnyEvent>> =
        Domain::new_in_memory_event_store::<AnyEvent>();
}

/// @covers: in_process_event_bus
#[test]
fn test_in_process_event_bus_factory_returns_working_bus() {
    use futures::executor::block_on;
    let bus = Domain::in_process_event_bus(EventBusConfig::default());
    block_on(async move {
        assert!(bus.publish(Arc::new(AnyEvent)).await.is_ok());
    });
}

/// @covers: noop_event_bus
#[test]
fn test_noop_event_bus_factory_returns_working_bus() {
    use futures::executor::block_on;
    let bus = Domain::noop_event_bus();
    block_on(async move {
        assert!(bus.publish(Arc::new(AnyEvent)).await.is_ok());
    });
}

/// @covers: reconstitute
#[test]
fn test_reconstitute_returns_none_for_unknown_id() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    #[derive(Default)]
    struct AnyAgg {
        id: String,
    }
    impl edge_domain::Aggregate for AnyAgg {
        type Event = AnyEvent;
        fn apply(&mut self, e: &AnyEvent) {
            self.id = e.aggregate_id().into();
        }
        fn id(&self) -> &str {
            &self.id
        }
    }
    let store = Domain::new_in_memory_event_store::<AnyEvent>();
    let result = rt
        .block_on(Domain::reconstitute::<AnyAgg>(&*store, "none"))
        .unwrap();
    assert!(result.is_none());
}

/// @covers: reconstitute
#[tokio::test]
async fn test_reconstitute_returns_none_for_empty_store() {
    #[derive(Default)]
    struct AnyAgg {
        id: String,
    }
    impl edge_domain::Aggregate for AnyAgg {
        type Event = AnyEvent;
        fn apply(&mut self, e: &AnyEvent) {
            self.id = e.aggregate_id().into();
        }
        fn id(&self) -> &str {
            &self.id
        }
    }
    let store = Domain::new_in_memory_event_store::<AnyEvent>();
    let result = Domain::reconstitute::<AnyAgg>(&*store, "none")
        .await
        .unwrap();
    assert!(result.is_none());
}
