//! [`HandlerBootstrap`] — constructor contract for handler implementations.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    BootstrapNameRequest, BootstrapNameResponse, HandlerBuildResponse,
};

/// Constructor contract for building typed handler implementations from config.
pub trait HandlerBootstrap {
    /// Returns a stable, non-empty identifier for this bootstrap implementation.
    fn bootstrap_name(
        &self,
        _req: BootstrapNameRequest,
    ) -> Result<BootstrapNameResponse, HandlerError> {
        Ok(BootstrapNameResponse { name: "handler" })
    }

    /// The configuration type used to construct this handler.
    type Config
    where
        Self: Sized;

    /// Build a handler from the given configuration.
    fn build(cfg: Self::Config) -> Result<HandlerBuildResponse<Self>, HandlerError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Cfg {
        valid: bool,
    }

    struct MyHandler;

    impl HandlerBootstrap for MyHandler {
        fn bootstrap_name(
            &self,
            _req: BootstrapNameRequest,
        ) -> Result<BootstrapNameResponse, HandlerError> {
            Ok(BootstrapNameResponse { name: "my_handler" })
        }

        type Config = Cfg;

        fn build(cfg: Cfg) -> Result<HandlerBuildResponse<Self>, HandlerError> {
            if cfg.valid {
                Ok(HandlerBuildResponse { handler: MyHandler })
            } else {
                Err(HandlerError::InvalidRequest("invalid config".into()))
            }
        }
    }

    #[test]
    fn test_build_valid_config_returns_ok_happy() {
        let result = MyHandler::build(Cfg { valid: true });
        assert!(result.is_ok());
        // Verify the result is actually the expected type
        let _h: MyHandler = result.unwrap().handler;
    }

    #[test]
    fn test_build_invalid_config_returns_err_error() {
        assert!(MyHandler::build(Cfg { valid: false }).is_err());
    }

    #[test]
    fn test_build_ok_is_named_type_edge() {
        // build returns the concrete type, not a trait object
        let _h: MyHandler = MyHandler::build(Cfg { valid: true }).unwrap().handler;
    }

    #[test]
    fn test_bootstrap_name_returns_nonempty_string_happy() {
        let h = MyHandler;
        assert!(!h
            .bootstrap_name(BootstrapNameRequest)
            .unwrap()
            .name
            .is_empty());
    }
}
