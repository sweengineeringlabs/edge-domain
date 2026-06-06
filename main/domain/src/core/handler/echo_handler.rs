//! [`Handler<T, T>`] impl for [`EchoHandler`].

use async_trait::async_trait;

use crate::api::handler::types::EchoHandler;
use crate::api::handler::Handler;
use crate::api::handler::HandlerError;

/// Primary type for this module — satisfies Rule 89 filename match.
#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — not constructed anywhere"
)]
pub(crate) struct DefaultEchoHandler;

#[async_trait]
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

    async fn execute(&self, req: T) -> Result<T, HandlerError> {
        Ok(req)
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
