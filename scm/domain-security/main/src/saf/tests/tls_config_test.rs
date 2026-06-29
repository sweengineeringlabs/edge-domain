//! Re-export verification for TLS SAF facade items.

#[cfg(test)]
mod tests {
    use crate::saf::TLS_CONFIG_SVC_FACTORY;

    /// @covers: TLS_CONFIG_SVC_FACTORY
    #[test]
    fn test_tls_config_svc_factory_is_unit_type_happy() {
        assert_eq!(TLS_CONFIG_SVC_FACTORY, (), "anchor must be unit type");
    }

    /// @covers: TLS_CONFIG_SVC_FACTORY
    #[test]
    fn test_tls_config_svc_factory_is_zero_sized_edge() {
        assert_eq!(
            std::mem::size_of_val(&TLS_CONFIG_SVC_FACTORY),
            0,
            "anchor must be zero-sized"
        );
    }
}
