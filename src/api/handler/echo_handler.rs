//! `EchoHandler` — returns the request as the response unchanged.

use std::marker::PhantomData;

/// A [`Handler`](super::Handler) that echoes its input as its output.
///
/// Useful for transport-layer integration tests — verifies routing and codec
/// wiring without requiring any business logic.
///
/// ```rust,ignore
/// let h: Arc<dyn Handler<String, String>> = echo_handler("ping", "/ping");
/// assert_eq!(h.execute("hello".into()).await.unwrap(), "hello");
/// ```
#[derive(Debug)]
pub struct EchoHandler<T> {
    pub(crate) id: String,
    pub(crate) pattern: String,
    pub(crate) _marker: PhantomData<fn(T) -> T>,
}

impl<T> EchoHandler<T> {
    /// Construct an `EchoHandler` with the given handler id and route pattern.
    pub fn new(id: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_handler_new_stores_id_and_pattern() {
        let h = EchoHandler::<String>::new("my-id", "/my-pattern");
        assert_eq!(h.id, "my-id");
        assert_eq!(h.pattern, "/my-pattern");
    }
}
