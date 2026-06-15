//! [`EchoHandler`] — a handler that reflects its request back as the response.

use std::marker::PhantomData;

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::traits::Handler;
use crate::api::handler::types::HandlerContext;

/// A handler that returns its request unchanged as the response.
///
/// Useful as a test double or default no-op implementation.
pub struct EchoHandler<T> {
    id: String,
    pattern: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T> EchoHandler<T> {
    /// Construct a new `EchoHandler` with the given id and pattern.
    pub fn new(id: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            _marker: PhantomData,
        }
    }

    /// Return the handler id string (accessor for tests that cannot call the trait method).
    pub fn id_str(&self) -> &str {
        &self.id
    }

    /// Return the pattern string (accessor for tests that cannot call the trait method).
    pub fn pattern_str(&self) -> &str {
        &self.pattern
    }
}

#[async_trait]
impl<T: Clone + Send + 'static> Handler for EchoHandler<T> {
    type Request = T;
    type Response = T;

    fn id(&self) -> &str {
        &self.id
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    async fn execute(&self, req: T, _ctx: HandlerContext<'_>) -> Result<T, HandlerError> {
        Ok(req)
    }
}
