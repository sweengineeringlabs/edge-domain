//! Integration tests for the [`TlsConfig`] trait contract.

use edge_domain_security::{IngressTlsConfig, IngressTlsError, TlsConfig};

struct TlsEndpoint {
    config: IngressTlsConfig,
}

impl TlsConfig for TlsEndpoint {
    fn ingress_tls(&self) -> &IngressTlsConfig {
        &self.config
    }

    fn validate_tls(&self) -> Result<(), IngressTlsError> {
        self.config.validate_tls()
    }
}

fn plain_tls(cert: &str, key: &str) -> TlsEndpoint {
    TlsEndpoint {
        config: IngressTlsConfig::tls(cert, key),
    }
}

fn mutual_tls(cert: &str, key: &str, ca: &str) -> TlsEndpoint {
    TlsEndpoint {
        config: IngressTlsConfig::mtls(cert, key, ca),
    }
}

/// @covers: TlsConfig::ingress_tls
#[test]
fn test_ingress_tls_returns_tls_config_happy() {
    let ep = plain_tls("cert.pem", "key.pem");
    assert_eq!(ep.ingress_tls().cert_pem_path, "cert.pem");
    assert_eq!(ep.ingress_tls().key_pem_path, "key.pem");
    assert!(ep.ingress_tls().client_ca_pem_path.is_none());
}

/// @covers: TlsConfig::ingress_tls
#[test]
fn test_ingress_tls_cert_path_reflects_empty_string_error() {
    let ep = plain_tls("", "key.pem");
    assert_eq!(
        ep.ingress_tls().cert_pem_path,
        "",
        "ingress_tls must return the stored config unchanged"
    );
}

/// @covers: TlsConfig::ingress_tls
#[test]
fn test_ingress_tls_returns_mtls_config_edge() {
    let ep = mutual_tls("cert.pem", "key.pem", "ca.pem");
    assert!(
        ep.ingress_tls().client_ca_pem_path.is_some(),
        "mTLS config must have client_ca_pem_path set"
    );
    assert_eq!(
        ep.ingress_tls().client_ca_pem_path.as_deref(),
        Some("ca.pem")
    );
}

/// @covers: TlsConfig::validate_tls
#[test]
fn test_validate_tls_valid_config_happy() {
    let result = plain_tls("cert.pem", "key.pem").validate_tls();
    assert!(result.is_ok(), "valid config must pass validation");
    assert_eq!(result.unwrap(), (), "successful validation returns unit");
}

/// @covers: TlsConfig::validate_tls
#[test]
fn test_validate_tls_empty_cert_path_error() {
    let result = plain_tls("", "key.pem").validate_tls();
    assert!(
        result.is_err(),
        "validate_tls must fail when cert path is empty"
    );
    assert!(
        matches!(result.unwrap_err(), IngressTlsError::Config(_)),
        "error must be IngressTlsError::Config"
    );
}

/// @covers: TlsConfig::validate_tls
#[test]
fn test_validate_tls_valid_mtls_config_edge() {
    let result = mutual_tls("cert.pem", "key.pem", "ca.pem").validate_tls();
    assert!(result.is_ok(), "valid mTLS config must pass validation");
    assert_eq!(
        result.unwrap(),
        (),
        "successful mTLS validation returns unit"
    );
}
