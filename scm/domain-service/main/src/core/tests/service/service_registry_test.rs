//! External tests for `ServiceRegistryStore`'s inherent/trait impls in core/.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::api::{
        NameRequest, NoopRequest, NoopResponse, NoopService, RegisterServiceRequest, Service,
        ServiceRegistryStore, StdServiceRegistryFactory,
    };

    /// @covers: default
    #[test]
    fn test_default_creates_empty_backing_map() {
        let store: ServiceRegistryStore<NoopRequest, NoopResponse> = Default::default();
        assert_eq!(store.inner.read().len(), 0);
    }

    /// @covers: default
    #[test]
    fn test_default_backing_map_accepts_insertion() {
        let store: ServiceRegistryStore<NoopRequest, NoopResponse> = Default::default();
        let svc: Arc<dyn Service<Request = NoopRequest, Response = NoopResponse>> =
            Arc::new(NoopService);
        store.inner.write().insert("noop".to_string(), svc);
        assert_eq!(store.inner.read().len(), 1);
    }

    /// @covers: new_registry
    #[test]
    fn test_new_registry_creates_empty_backing_map() {
        let store: ServiceRegistryStore<NoopRequest, NoopResponse> =
            StdServiceRegistryFactory::new_registry();
        assert_eq!(store.inner.read().len(), 0);
    }

    /// @covers: noop_service
    #[test]
    fn test_noop_service_reports_noop_name() {
        let svc = StdServiceRegistryFactory::noop_service();
        assert_eq!(svc.name(NameRequest).unwrap().name, "noop");
    }

    /// @covers: default_factory
    #[test]
    fn test_default_factory_returns_std_factory() {
        let factory = StdServiceRegistryFactory::default_factory();
        assert_eq!(factory, StdServiceRegistryFactory);
    }

    /// @covers: new
    #[test]
    fn test_new_wraps_given_service() {
        let req = RegisterServiceRequest::<NoopRequest, NoopResponse>::new(Arc::new(NoopService));
        assert_eq!(req.service.name(NameRequest).unwrap().name, "noop");
    }
}
