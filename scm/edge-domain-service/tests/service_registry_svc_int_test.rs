//! SAF facade tests — `ServiceRegistry` trait via `ServiceRegistry` struct.

use std::sync::Arc;

use edge_domain_service::{Service, ServiceError, ServiceRegistry, ServiceRegistryImpl};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Fixed(String, String);
impl Service<String, String> for Fixed {
    fn name(&self) -> &str {
        &self.0
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        let val = self.1.clone();
        Box::pin(async move { Ok(val) })
    }
}

fn make_registry() -> ServiceRegistry<String, String> {
    ServiceRegistry::new()
}

/// @covers: ServiceRegistry::register — service is findable after registration
#[test]
fn test_register_new_service_is_findable_happy() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("alpha".into(), "a".into())));
    assert!(reg.get("alpha").is_some());
}

/// @covers: ServiceRegistry::register — duplicate name overwrites previous
#[test]
fn test_register_duplicate_name_overwrites_entry_error() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("svc".into(), "v1".into())));
    reg.register(Arc::new(Fixed("svc".into(), "v2".into())));
    let result = block_on(
        reg.get("svc")
            .expect("service must exist")
            .execute("".into()),
    );
    assert_eq!(result.ok().as_deref(), Some("v2"));
}

/// @covers: ServiceRegistry::register — multiple distinct services coexist
#[test]
fn test_register_multiple_distinct_services_all_present_edge() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("a".into(), "1".into())));
    reg.register(Arc::new(Fixed("b".into(), "2".into())));
    reg.register(Arc::new(Fixed("c".into(), "3".into())));
    assert_eq!(reg.len(), 3);
}

/// @covers: ServiceRegistry::deregister — registered service is removed
#[test]
fn test_deregister_existing_service_returns_true_happy() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("x".into(), "v".into())));
    assert!(reg.deregister("x"));
    assert!(reg.get("x").is_none());
}

/// @covers: ServiceRegistry::deregister — absent name returns false
#[test]
fn test_deregister_absent_name_returns_false_error() {
    let reg = make_registry();
    assert!(!reg.deregister("ghost"));
}

/// @covers: ServiceRegistry::deregister — deregistering twice returns false second time
#[test]
fn test_deregister_twice_second_call_returns_false_edge() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("svc".into(), "v".into())));
    assert!(reg.deregister("svc"));
    assert!(!reg.deregister("svc"));
}

/// @covers: ServiceRegistry::get — present service returns Some
#[test]
fn test_get_present_service_returns_some_happy() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("present".into(), "ok".into())));
    assert!(reg.get("present").is_some());
}

/// @covers: ServiceRegistry::get — absent name returns None
#[test]
fn test_get_absent_name_returns_none_error() {
    let reg = make_registry();
    assert!(reg.get("missing").is_none());
}

/// @covers: ServiceRegistry::get — returned service is executable
#[test]
fn test_get_returned_service_is_executable_edge() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("exec".into(), "result".into())));
    let svc = reg.get("exec").expect("service must be present");
    let out = block_on(svc.execute("input".into()));
    assert_eq!(out.ok().as_deref(), Some("result"));
}

/// @covers: ServiceRegistry::list_names — returns all registered names
#[test]
fn test_list_names_returns_all_registered_names_happy() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("one".into(), "1".into())));
    reg.register(Arc::new(Fixed("two".into(), "2".into())));
    let mut names = reg.list_names();
    names.sort();
    assert_eq!(names, vec!["one", "two"]);
}

/// @covers: ServiceRegistry::list_names — empty registry returns empty vec
#[test]
fn test_list_names_empty_registry_returns_empty_error() {
    let reg = make_registry();
    assert!(reg.list_names().is_empty());
}

/// @covers: ServiceRegistry::list_names — reflects deregistration
#[test]
fn test_list_names_after_deregister_excludes_name_edge() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("keep".into(), "k".into())));
    reg.register(Arc::new(Fixed("drop".into(), "d".into())));
    reg.deregister("drop");
    assert_eq!(reg.list_names(), vec!["keep"]);
}

/// @covers: ServiceRegistry::len — reflects count
#[test]
fn test_len_after_registrations_returns_count_happy() {
    let reg = make_registry();
    assert_eq!(reg.len(), 0);
    reg.register(Arc::new(Fixed("a".into(), "1".into())));
    assert_eq!(reg.len(), 1);
    reg.register(Arc::new(Fixed("b".into(), "2".into())));
    assert_eq!(reg.len(), 2);
}

/// @covers: ServiceRegistry::len — deregistration decrements count
#[test]
fn test_len_after_deregister_decrements_count_error() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("a".into(), "1".into())));
    reg.deregister("a");
    assert_eq!(reg.len(), 0);
}

/// @covers: ServiceRegistry::len — empty registry has len zero
#[test]
fn test_len_empty_registry_returns_zero_edge() {
    let reg = make_registry();
    assert_eq!(reg.len(), 0);
}

/// @covers: ServiceRegistry::is_empty — non-empty registry is not empty
#[test]
fn test_is_empty_non_empty_registry_returns_false_happy() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("svc".into(), "v".into())));
    assert!(!reg.is_empty());
}

/// @covers: ServiceRegistry::is_empty — deregistering all services makes registry empty
#[test]
fn test_is_empty_after_all_deregistered_returns_true_error() {
    let reg = make_registry();
    reg.register(Arc::new(Fixed("only".into(), "v".into())));
    reg.deregister("only");
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistry::is_empty — fresh registry is empty
#[test]
fn test_is_empty_fresh_registry_returns_true_edge() {
    let reg = make_registry();
    assert!(reg.is_empty());
    reg.register(Arc::new(Fixed("svc".into(), "v".into())));
    assert!(!reg.is_empty());
}

/// @covers: ServiceRegistry trait impl — trait methods delegate to struct
#[test]
fn test_trait_impl_delegates_correctly_happy() {
    let reg = make_registry();
    ServiceRegistryImpl::register(&reg, Arc::new(Fixed("via_trait".into(), "ok".into())));
    assert_eq!(ServiceRegistryImpl::len(&reg), 1);
    assert!(ServiceRegistryImpl::get(&reg, "via_trait").is_some());
    assert!(ServiceRegistryImpl::deregister(&reg, "via_trait"));
    assert!(ServiceRegistryImpl::is_empty(&reg));
}
