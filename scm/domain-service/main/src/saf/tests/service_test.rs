//! External tests for the saf/service facade constants.

#[cfg(test)]
mod tests {
    /// @covers: SERVICE_SVC
    #[test]
    fn test_service_svc_identifier_value() {
        assert_eq!(crate::SERVICE_SVC, "service");
    }

    /// @covers: SERVICE_REGISTRY_SVC
    #[test]
    fn test_service_registry_svc_identifier_value() {
        assert_eq!(crate::SERVICE_REGISTRY_SVC, "service_registry");
    }

    /// @covers: SERVICE_SVC
    #[test]
    fn test_service_svc_distinct_from_registry_svc_edge() {
        assert_ne!(crate::SERVICE_SVC, crate::SERVICE_REGISTRY_SVC);
    }
}
