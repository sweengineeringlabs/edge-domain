//! `impl HandlerBootstrap for NoopHandlerFactory` — structural compliance impl.

use crate::api::HandlerBootstrap;
use crate::api::HandlerError;
use crate::api::NoopHandlerFactory;

impl HandlerBootstrap for NoopHandlerFactory {
    type Config = ();

    fn build(_: ()) -> Result<Self, HandlerError> {
        Ok(NoopHandlerFactory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_unit_config_returns_ok_happy() {
        let result = NoopHandlerFactory::build(());
        assert!(result.is_ok());
        let _: NoopHandlerFactory = result.unwrap();
    }

    #[test]
    fn test_build_constructs_noop_handler_factory_edge() {
        let _f: NoopHandlerFactory = NoopHandlerFactory::build(()).unwrap();
    }

    #[test]
    fn test_build_does_not_return_error_for_unit_config_error() {
        // The only possible config is (); there is no invalid input path.
        // This test documents that absence explicitly.
        let result = NoopHandlerFactory::build(());
        assert!(result.is_ok());
        let _: NoopHandlerFactory = result.unwrap();
    }
}
