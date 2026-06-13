//! `Service` trait impl for [`NoopService`] — a no-operation service.

use futures::future::BoxFuture;

use crate::api::service::errors::ServiceError;
use crate::api::service::traits::service::Service;
use crate::api::service::types::NoopService;

impl Service for NoopService {
    type Request = ();
    type Response = ();

    fn name(&self) -> &str {
        "noop"
    }

    fn execute(&self, _req: ()) -> BoxFuture<'_, Result<(), ServiceError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::api::service::traits::service::Service;
    use crate::api::service::types::NoopService;

    /// @covers: name
    #[test]
    fn test_name_noop_returns_noop_happy() {
        assert_eq!(NoopService.name(), "noop");
    }

    /// @covers: execute
    #[test]
    fn test_execute_noop_returns_ok_happy() {
        let result = block_on(NoopService.execute(()));
        assert!(result.is_ok());
    }

    /// @covers: execute
    #[test]
    fn test_execute_noop_never_errors_edge() {
        for _ in 0..3 {
            assert!(block_on(NoopService.execute(())).is_ok());
        }
    }
}
