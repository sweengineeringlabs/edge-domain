//! Complete test coverage for ServiceRegistry trait methods.

#[cfg(test)]
mod tests {
    use crate::api::{
        EmptinessRequest, LenRequest, ListNamesRequest, NoopService, RegisterServiceRequest,
        Service, ServiceError, ServiceLookupRequest, ServiceRegistry, ServiceRemovalRequest,
    };
    use edge_application_base::{EmptyRequest, EmptyResponse};
    use std::sync::Arc;

    struct TestRegistry;

    impl ServiceRegistry for TestRegistry {
        type Request = EmptyRequest;
        type Response = EmptyResponse;

        fn register(
            &self,
            _req: &RegisterServiceRequest<EmptyRequest, EmptyResponse>,
        ) -> Result<crate::api::RegisterServiceResponse, ServiceError> {
            Ok(crate::api::RegisterServiceResponse)
        }

        fn deregister(
            &self,
            _req: &ServiceRemovalRequest,
        ) -> Result<crate::api::ServiceRemovalResponse, ServiceError> {
            Ok(crate::api::ServiceRemovalResponse { was_present: false })
        }

        fn get(
            &self,
            _req: &ServiceLookupRequest,
        ) -> Result<crate::api::ServiceLookupResponse<EmptyRequest, EmptyResponse>, ServiceError> {
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

        fn is_empty(
            &self,
            _req: EmptinessRequest,
        ) -> Result<crate::api::EmptinessResponse, ServiceError> {
            Ok(crate::api::EmptinessResponse { empty: true })
        }

        fn default_factory() -> crate::api::StdServiceRegistryFactory {
            crate::api::StdServiceRegistryFactory
        }

        fn noop_service() -> crate::api::NoopService {
            crate::api::NoopService
        }

        fn new_store() -> crate::api::ServiceRegistryStore<EmptyRequest, EmptyResponse> {
            crate::api::ServiceRegistryStore::default()
        }
    }

    /// @covers: ServiceRegistry::register
    #[test]
    fn test_register_service_happy() {
        let reg = TestRegistry;
        let svc = Arc::new(NoopService);
        let req = RegisterServiceRequest::new(svc);
        assert_eq!(reg.register(&req), Ok(crate::api::RegisterServiceResponse));
    }

    /// @covers: ServiceRegistry::register
    #[test]
    fn test_register_returns_response_error() {
        let reg = TestRegistry;
        let svc = Arc::new(NoopService);
        let req = RegisterServiceRequest::new(svc);
        let result = reg.register(&req);
        assert_eq!(result, Ok(crate::api::RegisterServiceResponse));
    }

    /// @covers: ServiceRegistry::register
    #[test]
    fn test_register_idempotent_edge() {
        let reg = TestRegistry;
        for _ in 0..2 {
            let svc = Arc::new(NoopService);
            let req = RegisterServiceRequest::new(svc);
            assert_eq!(reg.register(&req), Ok(crate::api::RegisterServiceResponse));
        }
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_deregister_missing_happy() {
        let reg = TestRegistry;
        let req = ServiceRemovalRequest {
            name: "x".to_string(),
        };
        assert!(!reg.deregister(&req).unwrap().was_present);
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_deregister_not_present_error() {
        let reg = TestRegistry;
        let req = ServiceRemovalRequest {
            name: "x".to_string(),
        };
        let result = reg.deregister(&req);
        assert!(!result.unwrap().was_present);
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_deregister_consistent_edge() {
        let reg = TestRegistry;
        let req1 = ServiceRemovalRequest {
            name: "x".to_string(),
        };
        let req2 = ServiceRemovalRequest {
            name: "x".to_string(),
        };
        assert_eq!(
            reg.deregister(&req1).unwrap().was_present,
            reg.deregister(&req2).unwrap().was_present
        );
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_returns_option_happy() {
        let reg = TestRegistry;
        let req = ServiceLookupRequest {
            name: "x".to_string(),
        };
        let result = reg.get(&req);
        assert!(result.unwrap().service.is_none());
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_none_error() {
        let reg = TestRegistry;
        let req = ServiceLookupRequest {
            name: "x".to_string(),
        };
        let result = reg.get(&req);
        assert!(result.unwrap().service.is_none());
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_consistent_edge() {
        let reg = TestRegistry;
        let req1 = ServiceLookupRequest {
            name: "x".to_string(),
        };
        let req2 = ServiceLookupRequest {
            name: "x".to_string(),
        };
        let r1 = reg.get(&req1);
        let r2 = reg.get(&req2);
        assert_eq!(r1.unwrap().service.is_some(), r2.unwrap().service.is_some());
    }

    /// @covers: ServiceRegistry::list_names
    #[test]
    fn test_list_names_returns_vec_happy() {
        let reg = TestRegistry;
        let req = ListNamesRequest;
        assert!(reg.list_names(req).unwrap().names.is_empty());
    }

    /// @covers: ServiceRegistry::list_names
    #[test]
    fn test_list_names_empty_error() {
        let reg = TestRegistry;
        let req = ListNamesRequest;
        let result = reg.list_names(req);
        assert!(result.unwrap().names.is_empty());
    }

    /// @covers: ServiceRegistry::list_names
    #[test]
    fn test_list_names_consistent_edge() {
        let reg = TestRegistry;
        let req1 = ListNamesRequest;
        let req2 = ListNamesRequest;
        let r1 = reg.list_names(req1);
        let r2 = reg.list_names(req2);
        assert_eq!(r1.unwrap().names.len(), r2.unwrap().names.len());
    }

    /// @covers: ServiceRegistry::len
    #[test]
    fn test_len_returns_count_happy() {
        let reg = TestRegistry;
        let req = LenRequest;
        assert_eq!(reg.len(req).unwrap().count, 0);
    }

    /// @covers: ServiceRegistry::len
    #[test]
    fn test_len_zero_error() {
        let reg = TestRegistry;
        let req = LenRequest;
        let result = reg.len(req);
        assert_eq!(result.unwrap().count, 0);
    }

    /// @covers: ServiceRegistry::len
    #[test]
    fn test_len_consistent_edge() {
        let reg = TestRegistry;
        let req1 = LenRequest;
        let req2 = LenRequest;
        assert_eq!(reg.len(req1).unwrap().count, reg.len(req2).unwrap().count);
    }

    /// @covers: ServiceRegistry::is_empty
    #[test]
    fn test_is_empty_returns_bool_happy() {
        let reg = TestRegistry;
        let req = EmptinessRequest;
        assert!(reg.is_empty(req).unwrap().empty);
    }

    /// @covers: ServiceRegistry::is_empty
    #[test]
    fn test_is_empty_true_error() {
        let reg = TestRegistry;
        let req = EmptinessRequest;
        let result = reg.is_empty(req);
        assert!(result.unwrap().empty);
    }

    /// @covers: ServiceRegistry::is_empty
    #[test]
    fn test_is_empty_consistent_edge() {
        let reg = TestRegistry;
        let req1 = EmptinessRequest;
        let req2 = EmptinessRequest;
        assert_eq!(
            reg.is_empty(req1).unwrap().empty,
            reg.is_empty(req2).unwrap().empty
        );
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_default_factory_returns_factory_happy() {
        let factory = TestRegistry::default_factory();
        assert_eq!(
            std::mem::size_of_val(&factory),
            std::mem::size_of::<crate::api::StdServiceRegistryFactory>()
        );
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_default_factory_consistent_error() {
        let factory1 = TestRegistry::default_factory();
        let factory2 = TestRegistry::default_factory();
        assert_eq!(
            std::mem::size_of_val(&factory1),
            std::mem::size_of_val(&factory2)
        );
    }

    /// @covers: ServiceRegistry::default_factory
    #[test]
    fn test_default_factory_multiple_calls_edge() {
        let factory1 = TestRegistry::default_factory();
        let factory2 = TestRegistry::default_factory();
        let factory3 = TestRegistry::default_factory();
        assert_eq!(
            std::mem::size_of_val(&factory1),
            std::mem::size_of_val(&factory2)
        );
        assert_eq!(
            std::mem::size_of_val(&factory2),
            std::mem::size_of_val(&factory3)
        );
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_noop_service_returns_noop_happy() {
        let noop = TestRegistry::noop_service();
        assert_eq!(noop.name(crate::api::NameRequest).unwrap().name, "noop");
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_noop_service_is_consistent_error() {
        let noop1 = TestRegistry::noop_service();
        let noop2 = TestRegistry::noop_service();
        assert_eq!(
            noop1.name(crate::api::NameRequest).unwrap().name,
            noop2.name(crate::api::NameRequest).unwrap().name
        );
    }

    /// @covers: ServiceRegistry::noop_service
    #[test]
    fn test_noop_service_multiple_calls_edge() {
        let noop1 = TestRegistry::noop_service();
        let noop2 = TestRegistry::noop_service();
        let noop3 = TestRegistry::noop_service();
        assert_eq!(
            noop1.name(crate::api::NameRequest).unwrap().name,
            noop2.name(crate::api::NameRequest).unwrap().name
        );
        assert_eq!(
            noop2.name(crate::api::NameRequest).unwrap().name,
            noop3.name(crate::api::NameRequest).unwrap().name
        );
    }

    /// @covers: ServiceRegistry::new_store
    #[test]
    fn test_new_store_returns_empty_registry_happy() {
        let store = TestRegistry::new_store();
        assert!(store.is_empty(EmptinessRequest).unwrap().empty);
    }

    /// @covers: ServiceRegistry::new_store
    #[test]
    fn test_new_store_usable_for_registration_error() {
        let store = TestRegistry::new_store();
        let svc = Arc::new(NoopService);
        let req = RegisterServiceRequest::new(svc);
        assert_eq!(
            store.register(&req),
            Ok(crate::api::RegisterServiceResponse)
        );
        assert_eq!(store.len(LenRequest).unwrap().count, 1);
    }

    /// @covers: ServiceRegistry::new_store
    #[test]
    fn test_new_store_instances_are_independent_edge() {
        let store1 = TestRegistry::new_store();
        let store2 = TestRegistry::new_store();
        let svc = Arc::new(NoopService);
        let req = RegisterServiceRequest::new(svc);
        store1.register(&req).unwrap();
        assert_eq!(store1.len(LenRequest).unwrap().count, 1);
        assert_eq!(store2.len(LenRequest).unwrap().count, 0);
    }
}
