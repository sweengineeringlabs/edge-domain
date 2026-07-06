//! Service trait impl for [`NoopService`] — a no-operation service.

use futures::future::BoxFuture;

use crate::api::{NameRequest, NameResponse, NoopService, Service, ServiceError};

impl Service for NoopService {
    type Request = ();
    type Response = ();

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "noop".to_string(),
        })
    }

    fn execute(&self, _req: ()) -> BoxFuture<'_, Result<(), ServiceError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::api::{NameRequest, NameResponse, NoopService, Service};

    /// @covers: name
    #[test]
    fn test_name_noop_returns_noop_happy() {
        let result = NoopService.name(NameRequest);
        assert_eq!(
            result,
            Ok(NameResponse {
                name: "noop".to_string()
            })
        );
    }

    /// @covers: execute
    #[test]
    fn test_execute_noop_returns_ok_happy() {
        let result = block_on(NoopService.execute(()));
        assert_eq!(result, Ok(()));
    }

    /// @covers: execute
    #[test]
    fn test_execute_noop_never_errors_edge() {
        for _i in 0..3 {
            let result = block_on(NoopService.execute(()));
            assert_eq!(result, Ok(()));
        }
    }
}
