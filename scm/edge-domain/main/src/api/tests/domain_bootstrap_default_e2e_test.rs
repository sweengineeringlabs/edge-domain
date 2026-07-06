//! External tests for `DomainBootstrap`'s default trait method bodies.

#[cfg(test)]
mod tests {
    use crate::api::{Domain, DomainBootstrap, DomainBootstrapNameRequest};

    /// @covers: DomainBootstrap::bootstrap_name
    #[test]
    fn test_bootstrap_name_default_body_returns_domain_happy() {
        let response = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
        assert_eq!(response.name, "domain");
    }

    /// @covers: DomainBootstrap::bootstrap_name
    #[test]
    fn test_bootstrap_name_default_body_is_idempotent_edge() {
        let first = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
        let second = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
        assert_eq!(first.name, second.name);
    }

    /// @covers: DomainBootstrap::bootstrap_name
    #[test]
    fn test_bootstrap_name_default_body_callable_via_trait_object_error() {
        let f: &dyn DomainBootstrap = &Domain;
        let response = f.bootstrap_name(DomainBootstrapNameRequest).unwrap();
        assert_eq!(response.name, "domain");
    }
}
