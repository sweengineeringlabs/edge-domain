//! Integration tests for [`IngressTlsError`] — verifies each variant constructs correctly
//! and produces the expected error message.

use edge_domain_security::IngressTlsError;

/// @covers: CertLoad
#[test]
fn test_cert_load_error_contains_path_happy() {
    let err = IngressTlsError::CertLoad(
        "cert.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "no such file"),
    );
    assert!(
        err.to_string().contains("cert.pem"),
        "message must name the file path"
    );
    assert!(err.to_string().contains("load"), "message must say 'load'");
}

/// @covers: CertLoad
#[test]
fn test_cert_load_source_error_is_accessible_error() {
    use std::error::Error;
    let err = IngressTlsError::CertLoad(
        "cert.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied"),
    );
    let source = err
        .source()
        .expect("CertLoad must expose its io::Error as the error source");
    assert!(
        source.to_string().contains("access denied"),
        "source error must preserve the original io::Error message"
    );
}

/// @covers: CertLoad
#[test]
fn test_cert_load_path_with_directory_edge() {
    let err = IngressTlsError::CertLoad(
        "/etc/ssl/certs/server.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    );
    assert!(err.to_string().contains("/etc/ssl/certs/server.pem"));
}

/// @covers: CertParse
#[test]
fn test_cert_parse_error_contains_message_happy() {
    let err = IngressTlsError::CertParse("no certificates in PEM".to_string());
    assert!(
        err.to_string().contains("parse"),
        "message must say 'parse'"
    );
    assert!(err.to_string().contains("no certificates in PEM"));
}

/// @covers: CertParse
#[test]
fn test_cert_parse_error_is_not_a_load_error_error() {
    let err = IngressTlsError::CertParse("corrupt data".to_string());
    assert!(
        !err.to_string().contains("load"),
        "CertParse must not be confused with CertLoad"
    );
}

/// @covers: CertParse
#[test]
fn test_cert_parse_empty_message_edge() {
    let err = IngressTlsError::CertParse(String::new());
    assert!(
        err.to_string().contains("parse"),
        "message must still contain 'parse' for empty detail"
    );
}

/// @covers: KeyLoad
#[test]
fn test_key_load_error_contains_path_happy() {
    let err = IngressTlsError::KeyLoad(
        "key.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "missing key file"),
    );
    assert!(
        err.to_string().contains("key.pem"),
        "message must name the key file path"
    );
    assert!(
        err.to_string().contains("key"),
        "message must contain 'key'"
    );
}

/// @covers: KeyLoad
#[test]
fn test_key_load_source_error_is_accessible_error() {
    use std::error::Error;
    let err = IngressTlsError::KeyLoad(
        "key.pem".to_string(),
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied"),
    );
    let source = err
        .source()
        .expect("KeyLoad must expose its io::Error as the error source");
    assert!(
        source.to_string().contains("access denied"),
        "source error must preserve the original io::Error message"
    );
}

/// @covers: KeyLoad
#[test]
fn test_key_load_path_with_directory_edge() {
    let err = IngressTlsError::KeyLoad(
        "/etc/ssl/private/server.key".to_string(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    );
    assert!(err.to_string().contains("/etc/ssl/private/server.key"));
}

/// @covers: KeyParse
#[test]
fn test_key_parse_error_contains_message_happy() {
    let err = IngressTlsError::KeyParse("no private key in PEM".to_string());
    assert!(
        err.to_string().contains("parse"),
        "message must say 'parse'"
    );
    assert!(err.to_string().contains("no private key in PEM"));
}

/// @covers: KeyParse
#[test]
fn test_key_parse_error_is_not_a_load_error_error() {
    let err = IngressTlsError::KeyParse("corrupt data".to_string());
    assert!(
        !err.to_string().contains("load"),
        "KeyParse must not be confused with KeyLoad"
    );
}

/// @covers: KeyParse
#[test]
fn test_key_parse_empty_message_edge() {
    let err = IngressTlsError::KeyParse(String::new());
    assert!(
        err.to_string().contains("parse"),
        "message must still contain 'parse' for empty detail"
    );
}

/// @covers: Config
#[test]
fn test_config_error_contains_message_happy() {
    let err = IngressTlsError::Config("invalid TLS setting".to_string());
    assert!(
        err.to_string().contains("configuration"),
        "message must say 'configuration'"
    );
    assert!(err.to_string().contains("invalid TLS setting"));
}

/// @covers: Config
#[test]
fn test_config_error_does_not_expose_source_error() {
    use std::error::Error;
    let err = IngressTlsError::Config("cert path empty".to_string());
    assert!(
        err.source().is_none(),
        "Config variant has no underlying source error"
    );
}

/// @covers: Config
#[test]
fn test_config_error_empty_message_edge() {
    let err = IngressTlsError::Config(String::new());
    assert!(
        err.to_string().contains("configuration"),
        "message must still say 'configuration' for empty detail"
    );
}
