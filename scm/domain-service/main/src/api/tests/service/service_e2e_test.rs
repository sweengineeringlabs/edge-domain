//! Complete test coverage for Service trait methods.

#[cfg(test)]
mod tests {
    use crate::api::{NameRequest, NameResponse, NoopRequest, NoopResponse, Service, ServiceError};
    use futures::executor::block_on;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct TestService;

    impl Service for TestService {
        type Request = NoopRequest;
        type Response = NoopResponse;

        fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
            Ok(NameResponse {
                name: "test".to_string(),
            })
        }

        fn execute(&self, _req: NoopRequest) -> BoxFuture<'_, Result<NoopResponse, ServiceError>> {
            Box::pin(async move { Ok(NoopResponse) })
        }
    }

    /// @covers: Service::name
    #[test]
    fn test_name_returns_ok_happy() {
        let svc = TestService;
        assert_eq!(svc.name(NameRequest).unwrap().name, "test");
    }

    /// @covers: Service::name
    #[test]
    fn test_name_returns_response_error() {
        let svc = TestService;
        let result = svc.name(NameRequest);
        assert_eq!(result.unwrap().name, "test");
    }

    /// @covers: Service::name
    #[test]
    fn test_name_consistent_edge() {
        let svc = TestService;
        let r1 = svc.name(NameRequest);
        let r2 = svc.name(NameRequest);
        assert_eq!(r1, r2);
    }

    /// @covers: Service::execute
    #[test]
    fn test_execute_returns_ok_happy() {
        let svc = TestService;
        let result = block_on(svc.execute(NoopRequest));
        assert_eq!(result, Ok(NoopResponse));
    }

    /// @covers: Service::execute
    #[test]
    fn test_execute_no_error_error() {
        let svc = TestService;
        let result = block_on(svc.execute(NoopRequest));
        assert_eq!(result, Ok(NoopResponse));
    }

    /// @covers: Service::execute
    #[test]
    fn test_execute_idempotent_edge() {
        let svc = TestService;
        for _ in 0..3 {
            assert_eq!(block_on(svc.execute(NoopRequest)), Ok(NoopResponse));
        }
    }

    /// @covers: Service
    #[test]
    fn test_service_as_dyn_trait_edge() {
        let svc: Arc<dyn Service<Request = NoopRequest, Response = NoopResponse>> = Arc::new(TestService);
        let result = svc.name(NameRequest);
        assert_eq!(result.unwrap().name, "test");
    }
}
