//! SEA interface contract — primary traits for `edge-domain`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Handler`] | Business logic execution unit |
//! | [`Validator`] | Configuration validation contract |

/// Configuration validation contract.
///
/// Implemented by configuration types to validate their fields before use.
#[allow(dead_code)]
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self) -> Result<(), String> {
            Err("invalid".into())
        }
    }

    #[test]
    fn test_validator_ok_implementation_returns_ok() {
        assert!(AlwaysValid.validate().is_ok());
    }

    #[test]
    fn test_validator_err_implementation_returns_err() {
        assert!(AlwaysInvalid.validate().is_err());
    }

    #[test]
    fn test_handler_is_object_safe() {
        use crate::api::handler::Handler;
        fn _assert(_: &dyn Handler<String, String>) {}
    }
}
