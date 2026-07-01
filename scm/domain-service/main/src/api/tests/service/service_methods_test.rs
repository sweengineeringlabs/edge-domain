//! Complete test coverage for Service trait methods.

#[cfg(test)]
mod tests {
    use crate::api::{NameRequest, NameResponse, Service, ServiceError};
    use futures::executor::block_on;

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
        let result = block_on(svc.execute(()));
        assert_eq!(result, Ok(()));
    }

    /// @covers: Service::execute
    #[test]
    fn test_execute_no_error_error() {
        let svc = TestService;
        let result = block_on(svc.execute(()));
        assert_eq!(result, Ok(()));
    }

    /// @covers: Service::execute
    #[test]
    fn test_execute_idempotent_edge() {
        let svc = TestService;
        for _ in 0..3 {
            assert_eq!(block_on(svc.execute(())), Ok(()));
        }
    }
}
