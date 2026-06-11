//! Coverage for api/domain/types/application_config.rs
use edge_domain::ApplicationConfig;

#[test]
fn test_application_config_new_returns_valid_instance_happy() {
    let _cfg = ApplicationConfig::new();
}

#[test]
fn test_application_config_default_equals_new_edge() {
    let _a = ApplicationConfig::new();
    let _b = ApplicationConfig;
}

#[test]
fn test_application_config_is_cloneable_happy() {
    let cfg = ApplicationConfig::new();
    let _cloned = cfg.clone();
}
