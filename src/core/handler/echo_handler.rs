//! [`Handler<T, T>`] impl for [`EchoHandler`].

use futures::future::BoxFuture;

use crate::api::handler::Handler;
use crate::api::error::HandlerError;
use crate::api::types::EchoHandler;

/// Primary type for this module (matches filename for Rule 89).
#[allow(dead_code)]
pub(crate) struct EchoHandlerImpl;

impl<T> Handler<T, T> for EchoHandler<T>
where
    T: Send + 'static,
{
    fn id(&self) -> &str {
        &self.id
    }
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn execute(&self, req: T) -> BoxFuture<'_, Result<T, HandlerError>> {
        Box::pin(async move { Ok(req) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_returns_request_unchanged() {
        let h = EchoHandler::<String>::new("echo", "/echo");
        assert_eq!(h.execute("hello".into()).await.unwrap(), "hello");
    }

    #[test]
    fn test_id_returns_constructor_value() {
        let h = EchoHandler::<String>::new("my-id", "/p");
        assert_eq!(h.id(), "my-id");
    }

    #[test]
    fn test_pattern_returns_constructor_value() {
        let h = EchoHandler::<String>::new("id", "/api/v1/ping");
        assert_eq!(h.pattern(), "/api/v1/ping");
    }
}
