//! Contract tests for the [`TlsConfig`] default-method surface.

#[cfg(test)]
mod tests {
    use crate::api::{PemTlsConfig, TlsConfig};

    /// @covers: is_mtls
    #[test]
    fn test_is_mtls_false_for_tls_only_happy() {
        assert!(!PemTlsConfig::tls("cert.pem", "key.pem").is_mtls());
    }

    /// @covers: is_mtls
    #[test]
    fn test_is_mtls_true_when_ca_set_edge() {
        assert!(PemTlsConfig::mtls("cert.pem", "key.pem", "ca.pem").is_mtls());
    }

    /// @covers: cert_path
    #[test]
    fn test_cert_path_returns_configured_value_happy() {
        assert_eq!(
            PemTlsConfig::tls("server.crt", "server.key").cert_path(),
            "server.crt"
        );
    }

    /// @covers: key_path
    #[test]
    fn test_key_path_returns_configured_value_happy() {
        assert_eq!(
            PemTlsConfig::tls("server.crt", "server.key").key_path(),
            "server.key"
        );
    }

    /// @covers: ca_path
    #[test]
    fn test_ca_path_none_for_plain_tls_happy() {
        assert_eq!(
            PemTlsConfig::tls("cert.pem", "key.pem").ca_path(),
            None
        );
    }

    /// @covers: ca_path
    #[test]
    fn test_ca_path_some_for_mtls_edge() {
        assert_eq!(
            PemTlsConfig::mtls("c.pem", "k.pem", "ca.pem").ca_path(),
            Some("ca.pem")
        );
    }
}
