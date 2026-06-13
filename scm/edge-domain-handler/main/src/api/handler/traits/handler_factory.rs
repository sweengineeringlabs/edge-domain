//! [`HandlerFactory`] — constructor contract for handler implementations.

use crate::api::handler::errors::HandlerError;

/// Constructor contract for building typed handler implementations from config.
pub trait HandlerFactory: Sized {
    /// The configuration type used to construct this handler.
    type Config;

    /// Build a handler from the given configuration.
    fn build(cfg: Self::Config) -> Result<Self, HandlerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Cfg {
        valid: bool,
    }

    struct MyHandler;

    impl HandlerFactory for MyHandler {
        type Config = Cfg;

        fn build(cfg: Cfg) -> Result<Self, HandlerError> {
            if cfg.valid {
                Ok(MyHandler)
            } else {
                Err(HandlerError::InvalidRequest("invalid config".into()))
            }
        }
    }

    #[test]
    fn test_build_valid_config_returns_ok_happy() {
        assert!(MyHandler::build(Cfg { valid: true }).is_ok());
    }

    #[test]
    fn test_build_invalid_config_returns_err_error() {
        assert!(MyHandler::build(Cfg { valid: false }).is_err());
    }

    #[test]
    fn test_build_ok_is_named_type_edge() {
        // build returns the concrete type, not a trait object
        let _h: MyHandler = MyHandler::build(Cfg { valid: true }).unwrap();
    }
}
