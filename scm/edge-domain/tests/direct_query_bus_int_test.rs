//! Coverage for api/query/direct_query_bus.rs and api/query/types/direct_query_bus.rs
use edge_domain::{DirectQueryBus, Domain, QueryBus};
use std::sync::Arc;

#[test]
fn test_direct_query_bus_marker_type_is_constructible() {
    let _marker = DirectQueryBus;
}

#[test]
fn test_direct_query_bus_factory_returns_arc_query_bus() {
    let _: Arc<dyn QueryBus<String>> = Domain::direct_query_bus();
}

#[test]
fn test_direct_query_bus_factory_usable_as_query_bus_trait_object() {
    let bus: Arc<dyn QueryBus<String>> = Domain::direct_query_bus();
    drop(bus);
}
