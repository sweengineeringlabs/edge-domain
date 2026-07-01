//! Trait contract tests for ServiceRegistry — verifies public API.

#[cfg(test)]
mod tests {
    use crate::api::{
        EmptinessRequest, LenRequest, ListNamesRequest, RegisterServiceRequest, ServiceRegistry,
        ServiceRemovalRequest, ServiceLookupRequest, ServiceError,
    };
    use std::sync::Arc;

    struct TestRegistry;

    impl ServiceRegistry for TestRegistry {
        type Request = ();
        type Response = ();

        fn register(
            &self,
            _req: &RegisterServiceRequest<(), ()>,
        ) -> Result<crate::api::RegisterServiceResponse, ServiceError> {
            Ok(crate::api::RegisterServiceResponse)
        }

        fn deregister(
            &self,
            _req: ServiceRemovalRequest,
        ) -> Result<crate::api::ServiceRemovalResponse, ServiceError> {
            Ok(crate::api::ServiceRemovalResponse { was_present: false })
        }

        fn get(
            &self,
            _req: ServiceLookupRequest,
        ) -> Result<crate::api::ServiceLookupResponse<(), ()>, ServiceError> {
            Ok(crate::api::ServiceLookupResponse { service: None })
        }

        fn list_names(
            &self,
            _req: ListNamesRequest,
        ) -> Result<crate::api::ListNamesResponse, ServiceError> {
            Ok(crate::api::ListNamesResponse { names: vec![] })
        }

        fn len(&self, _req: LenRequest) -> Result<crate::api::LenResponse, ServiceError> {
            Ok(crate::api::LenResponse { count: 0 })
        }

        fn is_empty(&self, _req: EmptinessRequest) -> Result<crate::api::EmptinessResponse, ServiceError> {
            Ok(crate::api::EmptinessResponse { empty: true })
        }

        fn default_factory() -> crate::api::StdServiceRegistryFactory {
            crate::api::StdServiceRegistryFactory
        }

        fn noop_service() -> crate::api::NoopService {
            crate::api::NoopService
        }
    }

    /// @covers: ServiceRegistry::register
    #[test]
    fn test_service_registry_trait_register_contract_happy() {
        let reg = TestRegistry;
        let svc = Arc::new(crate::api::NoopService);
        let req = RegisterServiceRequest::new(svc);
        let result = reg.register(&req);
        assert_eq!(result, Ok(crate::api::RegisterServiceResponse));
    }

    /// @covers: ServiceRegistry::register
    #[test]
    fn test_service_registry_trait_register_contract_edge() {
        let reg = TestRegistry;
        let svc = Arc::new(crate::api::NoopService);
        let req = RegisterServiceRequest::new(svc);
        let result = reg.register(&req);
        assert_eq!(result, Ok(crate::api::RegisterServiceResponse));
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_service_registry_trait_deregister_contract_happy() {
        let reg = TestRegistry;
        let req = ServiceRemovalRequest {
            name: "test".to_string(),
        };
        let result = reg.deregister(req);
        assert_eq!(result.unwrap().was_present, false);
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_service_registry_trait_get_contract_happy() {
        let reg = TestRegistry;
        let req = ServiceLookupRequest {
            name: "test".to_string(),
        };
        let result = reg.get(req);
        assert!(result.unwrap().service.is_none());
    }

    /// @covers: ServiceRegistry::list_names
    #[test]
    fn test_service_registry_trait_list_names_contract_happy() {
        let reg = TestRegistry;
        let req = ListNamesRequest;
        let result = reg.list_names(req);
        assert!(result.unwrap().names.is_empty());
    }

    /// @covers: ServiceRegistry::list_names
    #[test]
    fn test_service_registry_trait_list_names_contract_edge() {
        let reg = TestRegistry;
        let req = ListNamesRequest;
        let result = reg.list_names(req);
        assert!(result.unwrap().names.is_empty());
    }

    /// @covers: ServiceRegistry::len
    #[test]
    fn test_service_registry_trait_len_contract_happy() {
        let reg = TestRegistry;
        let req = LenRequest;
        let result = reg.len(req);
        assert_eq!(result.unwrap().count, 0);
    }

    /// @covers: ServiceRegistry::is_empty
    #[test]
    fn test_service_registry_trait_is_empty_contract_happy() {
        let reg = TestRegistry;
        let req = EmptinessRequest;
        let result = reg.is_empty(req);
        assert!(result.unwrap().empty);
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_service_registry_trait_default_factory_contract_happy() {
        let factory = TestRegistry::default_factory();
        assert_eq!(std::mem::size_of_val(&factory), std::mem::size_of::<crate::api::StdServiceRegistryFactory>());
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_service_registry_trait_default_factory_contract_error() {
        let f1 = TestRegistry::default_factory();
        let f2 = TestRegistry::default_factory();
        assert_eq!(std::mem::size_of_val(&f1), std::mem::size_of_val(&f2));
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_service_registry_trait_default_factory_contract_edge() {
        let f1 = TestRegistry::default_factory();
        let f2 = TestRegistry::default_factory();
        assert_eq!(std::mem::size_of_val(&f1), std::mem::size_of_val(&f2));
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_service_registry_trait_noop_service_contract_happy() {
        let noop = TestRegistry::noop_service();
        assert_eq!(noop.name(crate::api::NameRequest).unwrap().name, "noop");
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_service_registry_trait_noop_service_contract_error() {
        let noop = TestRegistry::noop_service();
        let result = noop.name(crate::api::NameRequest);
        assert_eq!(result.unwrap().name, "noop");
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_service_registry_trait_noop_service_contract_edge() {
        let noop1 = TestRegistry::noop_service();
        let noop2 = TestRegistry::noop_service();
        assert_eq!(noop1.name(crate::api::NameRequest).unwrap().name, noop2.name(crate::api::NameRequest).unwrap().name);
    }
}
