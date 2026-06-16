//! [`Validator`] impl for [`AlwaysValid`].

use crate::api::validator::traits::Validator;
use crate::api::validator::types::AlwaysValid;

impl Validator for AlwaysValid {
    fn validate(&self) -> Result<(), crate::api::validator::errors::ValidatorError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_always_returns_ok() {
        assert!(AlwaysValid.validate().is_ok());
    }
}
