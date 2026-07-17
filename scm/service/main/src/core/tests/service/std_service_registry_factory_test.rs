//! External tests for `StdServiceRegistryFactory`'s public functions.

#[cfg(test)]
mod tests {
    use crate::api::{
        EmptinessRequest, NameRequest, Service, ServiceRegistry, ServiceRegistryStore,
        StdServiceRegistryFactory,
    };
    use edge_application_base::{EmptyRequest, EmptyResponse};

    /// @covers: new_registry
    #[test]
    fn test_new_registry_produces_usable_empty_registry_happy() {
        let reg: ServiceRegistryStore<EmptyRequest, EmptyResponse> =
            StdServiceRegistryFactory::new_registry();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    /// @covers: noop_service
    #[test]
    fn test_noop_service_produces_working_service_happy() {
        let svc = StdServiceRegistryFactory::noop_service();
        assert_eq!(svc.name(NameRequest).unwrap().name, "noop");
    }

    /// @covers: default_factory
    #[test]
    fn test_default_factory_produces_equal_factory_happy() {
        let factory = StdServiceRegistryFactory::default_factory();
        assert_eq!(factory, StdServiceRegistryFactory);
    }

    /// @covers: new_registry
    #[test]
    fn test_new_registry_instances_are_independent_edge() {
        let reg1: ServiceRegistryStore<EmptyRequest, EmptyResponse> =
            StdServiceRegistryFactory::new_registry();
        let reg2: ServiceRegistryStore<EmptyRequest, EmptyResponse> =
            StdServiceRegistryFactory::new_registry();
        assert!(reg1.is_empty(EmptinessRequest).unwrap().empty);
        assert!(reg2.is_empty(EmptinessRequest).unwrap().empty);
    }

    /// @covers: noop_service
    #[test]
    fn test_noop_service_multiple_calls_consistent_edge() {
        let svc1 = StdServiceRegistryFactory::noop_service();
        let svc2 = StdServiceRegistryFactory::noop_service();
        assert_eq!(
            svc1.name(NameRequest).unwrap().name,
            svc2.name(NameRequest).unwrap().name
        );
    }
}
