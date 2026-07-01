//! Trait contract tests for Service — verifies public API.

#[cfg(test)]
mod tests {
    use crate::api::{NameRequest, NameResponse, Service, ServiceError};
    use std::sync::Arc;

    struct TestService;

    impl Service for TestService {
        type Request = ();
        type Response = ();

        fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
            Ok(NameResponse {
                name: "test".to_string(),
            })
        }

        fn execute(&self, _req: ()) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), ServiceError>> + '_>,
        > {
            Box::pin(async move { Ok(()) })
        }
    }

    /// @covers: Service::name
    #[test]
    fn test_service_trait_name_contract_happy() {
        let svc = TestService;
        let result = svc.name(NameRequest);
        assert_eq!(result.unwrap().name, "test");
    }

    /// @covers: Service::execute
    #[test]
    fn test_service_trait_execute_contract_happy() {
        let svc = TestService;
        let future = svc.execute(());
        let result = futures::executor::block_on(future);
        assert_eq!(result, Ok(()));
    }

    /// @covers: Service
    #[test]
    fn test_service_as_dyn_trait_edge() {
        let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(TestService);
        let result = svc.name(NameRequest);
        assert_eq!(result.unwrap().name, "test");
    }
}
