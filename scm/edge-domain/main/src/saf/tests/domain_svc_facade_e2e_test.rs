//! External tests proving the `Domain` facade surface is reachable via saf/.

#[cfg(test)]
mod tests {
    use crate::api::Domain;

    /// @covers: DOMAIN_SVC
    #[test]
    fn test_domain_svc_anchor_is_accessible_happy() {
        assert_eq!(crate::saf::DOMAIN_SVC, ());
    }

    /// @covers: Domain.paired
    #[test]
    fn test_domain_paired_shares_backend_edge() {
        use std::sync::Arc;
        let backend = Arc::new(5i32);
        let (a, b) = Domain.paired(backend, |b| *b, |b| *b);
        assert_eq!(a, b);
    }

    /// @covers: Domain.paired
    #[test]
    fn test_domain_paired_independent_backends_produce_independent_results_error() {
        use std::sync::Arc;
        let (a, _) = Domain.paired(Arc::new(1i32), |b| *b, |b| *b);
        let (b, _) = Domain.paired(Arc::new(2i32), |b| *b, |b| *b);
        assert_ne!(a, b);
    }
}
