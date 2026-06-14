//! [`EchoHandler`] — returns its input unchanged.

use async_trait::async_trait;

use crate::api::handler::Handler;
use crate::api::handler::HandlerError;

/// A [`Handler`] that returns its request as its response without modification.
///
/// Use in transport-layer integration tests to verify routing and codec wiring
/// without requiring any business logic implementation.
///
/// # Examples
///
/// ```rust,no_run
/// use edge_domain::EchoHandler;
///
/// let h = EchoHandler::<String>::new("echo", "/ping");
/// ```
pub struct EchoHandler<T> {
    id: String,
    pattern: String,
    // fn() -> T is Send + Sync regardless of T.
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> EchoHandler<T> {
    /// Create an `EchoHandler` with the given `id` and routing `pattern`.
    pub fn new(id: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T: Send + 'static> Handler for EchoHandler<T> {
    type Request = T;
    type Response = T;

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
