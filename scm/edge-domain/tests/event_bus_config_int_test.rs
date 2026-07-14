//! Coverage for api/event/types/event/event_bus_config.rs
use edge_application::EventBusConfig;

#[test]
fn test_event_bus_config_default_capacity_is_1024() {
    let cfg = EventBusConfig::default();
    assert_eq!(cfg.capacity, 1024);
}

#[test]
fn test_event_bus_config_is_cloneable() {
    let cfg = EventBusConfig::default();
    let clone = cfg.clone();
    assert_eq!(clone.capacity, cfg.capacity);
}

#[test]
fn test_event_bus_config_custom_capacity_is_preserved() {
    let cfg = EventBusConfig { capacity: 64 };
    assert_eq!(cfg.capacity, 64);
}
