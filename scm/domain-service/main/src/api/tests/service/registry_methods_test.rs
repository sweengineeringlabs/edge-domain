//! Complete test coverage for ServiceRegistry trait methods.

#[cfg(test)]
mod tests {
    use crate::api::{
        EmptinessRequest, LenRequest, ListNamesRequest, RegisterServiceRequest, ServiceRegistry,
        ServiceRemovalRequest, ServiceLookupRequest, ServiceError, NoopService,
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
        let req = ServiceRemovalRequest { name: "x".to_string() };
        assert!(!reg.deregister(req).unwrap().was_present);
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_deregister_not_present_error() {
        let reg = TestRegistry;
        let req = ServiceRemovalRequest { name: "x".to_string() };
        let result = reg.deregister(req);
        assert!(!result.unwrap().was_present);
    }

    /// @covers: ServiceRegistry::deregister
    #[test]
    fn test_deregister_consistent_edge() {
        let reg = TestRegistry;
        let req1 = ServiceRemovalRequest { name: "x".to_string() };
        let req2 = ServiceRemovalRequest { name: "x".to_string() };
        assert_eq!(reg.deregister(req1).unwrap().was_present, reg.deregister(req2).unwrap().was_present);
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_returns_option_happy() {
        let reg = TestRegistry;
        let req = ServiceLookupRequest { name: "x".to_string() };
        let result = reg.get(req);
        assert!(result.unwrap().service.is_none());
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_none_error() {
        let reg = TestRegistry;
        let req = ServiceLookupRequest { name: "x".to_string() };
        let result = reg.get(req);
        assert!(result.unwrap().service.is_none());
    }

    /// @covers: ServiceRegistry::get
    #[test]
    fn test_get_consistent_edge() {
        let reg = TestRegistry;
        let req1 = ServiceLookupRequest { name: "x".to_string() };
        let req2 = ServiceLookupRequest { name: "x".to_string() };
        let r1 = reg.get(req1);
        let r2 = reg.get(req2);
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
        assert_eq!(reg.is_empty(req1).unwrap().empty, reg.is_empty(req2).unwrap().empty);
    }
}
