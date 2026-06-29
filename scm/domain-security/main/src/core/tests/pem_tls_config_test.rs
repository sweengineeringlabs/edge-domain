//! Colocated tests for [`PemTlsConfig`] public constructors.

#[cfg(test)]
mod tests {
    use crate::api::PemTlsConfig;

    /// @covers: tls
    #[test]
    fn test_tls_produces_no_ca_path_happy() {
        let cfg = PemTlsConfig::tls("cert.pem", "key.pem");
        assert_eq!(cfg.cert_pem_path, "cert.pem");
        assert_eq!(cfg.key_pem_path, "key.pem");
        assert_eq!(cfg.ca_pem_path, None);
    }

    /// @covers: mtls
    #[test]
    fn test_mtls_sets_ca_path_edge() {
        let cfg = PemTlsConfig::mtls("cert.pem", "key.pem", "ca.pem");
        assert_eq!(cfg.cert_pem_path, "cert.pem");
        assert_eq!(cfg.key_pem_path, "key.pem");
        assert_eq!(cfg.ca_pem_path, Some("ca.pem".to_string()));
    }
}
