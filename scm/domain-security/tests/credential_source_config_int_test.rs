//! Integration tests for [`CredentialSourceConfig`] type.

use edge_domain_security::CredentialSourceConfig;

#[test]
fn test_credential_source_config_default_happy() {
    let config = CredentialSourceConfig::default();
    assert_eq!(config, CredentialSourceConfig::default(), "Default config must be equal");
}

#[test]
fn test_credential_source_config_clone_happy() {
    let config = CredentialSourceConfig {
        env_var: Some("TEST".to_string()),
        file_path: None,
        file_path_env_override: None,
    };
    let cloned = config.clone();
    assert_eq!(config, cloned, "Cloned config must equal original");
}

#[test]
fn test_credential_source_config_env_var_field_happy() {
    let config = CredentialSourceConfig {
        env_var: Some("VAR".to_string()),
        file_path: None,
        file_path_env_override: None,
    };
    assert_eq!(config.env_var, Some("VAR".to_string()), "env_var field must be accessible");
}

#[test]
fn test_credential_source_config_file_path_field_happy() {
    let config = CredentialSourceConfig {
        env_var: None,
        file_path: Some("/path".to_string()),
        file_path_env_override: None,
    };
    assert_eq!(config.file_path, Some("/path".to_string()), "file_path field must be accessible");
}

#[test]
fn test_credential_source_config_all_fields_happy() {
    let config = CredentialSourceConfig {
        env_var: Some("VAR".to_string()),
        file_path: Some("/path".to_string()),
        file_path_env_override: Some("OVERRIDE".to_string()),
    };
    assert_eq!(config.env_var, Some("VAR".to_string()), "env_var must be set to VAR");
    assert_eq!(config.file_path, Some("/path".to_string()), "file_path must be set to /path");
    assert_eq!(config.file_path_env_override, Some("OVERRIDE".to_string()), "file_path_env_override must be set to OVERRIDE");
}

#[test]
fn test_credential_source_config_debug_happy() {
    let config = CredentialSourceConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
}

#[test]
fn test_credential_source_config_none_fields_edge() {
    let config = CredentialSourceConfig::default();
    assert!(config.env_var.is_none(), "Default env_var must be None");
    assert!(config.file_path.is_none(), "Default file_path must be None");
    assert!(config.file_path_env_override.is_none(), "Default file_path_env_override must be None");
}
