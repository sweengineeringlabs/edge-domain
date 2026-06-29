//! Integration tests for [`PemTlsConfig`] via the [`TlsConfig`] trait.

use edge_domain_security::{IngressTlsError, PemTlsConfig, TlsConfig};

struct TlsEndpoint {
    config: PemTlsConfig,
}

impl TlsConfig for TlsEndpoint {
    fn ingress_tls(&self) -> &PemTlsConfig {
        &self.config
    }

    fn validate_tls(&self) -> Result<(), IngressTlsError> {
        self.config.validate_tls()
    }
}

fn tls(cert: &str, key: &str) -> TlsEndpoint {
    TlsEndpoint {
        config: PemTlsConfig::tls(cert, key),
    }
}

fn mtls(cert: &str, key: &str, ca: &str) -> TlsEndpoint {
    TlsEndpoint {
        config: PemTlsConfig::mtls(cert, key, ca),
    }
}

/// @covers: tls
#[test]
fn test_pem_tls_config_tls_happy() {
    let ep = tls("cert.pem", "key.pem");
    assert_eq!(ep.cert_path(), "cert.pem");
    assert_eq!(ep.key_path(), "key.pem");
    assert!(!ep.is_mtls());
    assert_eq!(ep.ca_path(), None);
}

/// @covers: mtls
#[test]
fn test_pem_tls_config_mtls_happy() {
    let ep = mtls("cert.pem", "key.pem", "ca.pem");
    assert_eq!(ep.cert_path(), "cert.pem");
    assert_eq!(ep.key_path(), "key.pem");
    assert!(ep.is_mtls());
    assert_eq!(ep.ca_path(), Some("ca.pem"));
}

/// @covers: IngressTlsError
#[test]
fn test_ingress_tls_error_variants_edge() {
    let err_load = IngressTlsError::CertLoad(
        "cert.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    );
    assert!(err_load.to_string().contains("cert.pem"));

    let err_parse = IngressTlsError::CertParse("no certs in PEM".to_string());
    assert!(err_parse.to_string().contains("parse"));

    let err_key = IngressTlsError::KeyLoad(
        "key.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
    );
    assert!(err_key.to_string().contains("key.pem"));

    let err_config = IngressTlsError::Config("invalid".to_string());
    assert!(err_config.to_string().contains("configuration"));
}

/// @covers: is_mtls
#[test]
fn test_is_mtls_tls_config_returns_false_happy() {
    assert!(
        !tls("c.pem", "k.pem").is_mtls(),
        "plain TLS must not report is_mtls"
    );
}

/// @covers: is_mtls
#[test]
fn test_is_mtls_reports_false_for_invalid_tls_config_error() {
    assert!(
        !tls("", "").is_mtls(),
        "is_mtls must be false even for an invalid TLS config"
    );
}

/// @covers: is_mtls
#[test]
fn test_is_mtls_mtls_config_returns_true_edge() {
    assert!(
        mtls("c.pem", "k.pem", "ca.pem").is_mtls(),
        "mTLS config must report is_mtls"
    );
}

/// @covers: cert_path
#[test]
fn test_cert_path_returns_stored_value_happy() {
    assert_eq!(tls("my-cert.pem", "k.pem").cert_path(), "my-cert.pem");
}

/// @covers: cert_path
#[test]
fn test_cert_path_returns_empty_string_when_unset_error() {
    assert_eq!(
        tls("", "k.pem").cert_path(),
        "",
        "cert_path must return stored empty string"
    );
}

/// @covers: cert_path
#[test]
fn test_cert_path_returns_path_with_directory_edge() {
    assert_eq!(
        tls("/etc/ssl/certs/my.pem", "k.pem").cert_path(),
        "/etc/ssl/certs/my.pem"
    );
}

/// @covers: key_path
#[test]
fn test_key_path_returns_stored_value_happy() {
    assert_eq!(tls("c.pem", "my-key.pem").key_path(), "my-key.pem");
}

/// @covers: key_path
#[test]
fn test_key_path_returns_empty_string_when_unset_error() {
    assert_eq!(
        tls("c.pem", "").key_path(),
        "",
        "key_path must return stored empty string"
    );
}

/// @covers: key_path
#[test]
fn test_key_path_returns_path_with_directory_edge() {
    assert_eq!(
        tls("c.pem", "/etc/ssl/private/my.pem").key_path(),
        "/etc/ssl/private/my.pem"
    );
}

/// @covers: ca_path
#[test]
fn test_ca_path_none_for_tls_happy() {
    assert_eq!(tls("c.pem", "k.pem").ca_path(), None);
}

/// @covers: ca_path
#[test]
fn test_ca_path_returns_empty_string_when_empty_ca_set_error() {
    assert_eq!(
        mtls("c.pem", "k.pem", "").ca_path(),
        Some(""),
        "ca_path must return the stored empty CA string"
    );
}

/// @covers: ca_path
#[test]
fn test_ca_path_some_for_mtls_edge() {
    assert_eq!(mtls("c.pem", "k.pem", "ca.pem").ca_path(), Some("ca.pem"));
}

/// @covers: validate
#[test]
fn test_validate_valid_config_happy() {
    let result = tls("cert.pem", "key.pem").validate_tls();
    assert!(result.is_ok(), "valid config must pass validation");
    assert_eq!(result.unwrap(), (), "successful validation returns unit");
}

/// @covers: validate
#[test]
fn test_validate_empty_cert_path_error() {
    let result = tls("", "key.pem").validate_tls();
    assert!(
        matches!(result, Err(IngressTlsError::Config(_))),
        "empty cert path must return Config error"
    );
}

/// @covers: validate
#[test]
fn test_validate_empty_ca_path_in_mtls_edge() {
    let result = mtls("c.pem", "k.pem", "").validate_tls();
    assert!(
        matches!(result, Err(IngressTlsError::Config(_))),
        "empty CA path in mTLS must return Config error"
    );
}
