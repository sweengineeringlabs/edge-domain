//! `Handler` impl and construction for [`EchoHandler`].

use std::marker::PhantomData;

use async_trait::async_trait;

use crate::api::EchoHandler;
use crate::api::Handler;
use crate::api::HandlerContext;
use crate::api::HandlerError;

impl<T> From<(&str, &str)> for EchoHandler<T> {
    fn from((id, pattern): (&str, &str)) -> Self {
        Self { id: id.into(), pattern: pattern.into(), _marker: PhantomData }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_stores_id_and_pattern_happy() {
        let h = EchoHandler::<String>::from(("from-echo", "/from"));
        assert_eq!(h.id, "from-echo");
        assert_eq!(h.pattern, "/from");
    }

    #[test]
    fn test_from_empty_id_stores_empty_edge() {
        let h = EchoHandler::<String>::from(("", ""));
        assert_eq!(h.id, "");
    }

    #[test]
    fn test_from_empty_id_fails_handler_id_contract_error() {
        let h = EchoHandler::<String>::from(("", "/"));
        assert_eq!(h.id, "");
    }
}
