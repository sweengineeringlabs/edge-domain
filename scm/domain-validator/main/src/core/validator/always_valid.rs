//! [`Validator`] impl for [`AlwaysValid`].

use crate::api::Validator;
use crate::api::AlwaysValid;

impl Validator for AlwaysValid {
    fn validate(&self) -> Result<(), crate::api::ValidatorError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_always_returns_ok() {
        let result = AlwaysValid.validate();
        assert_eq!(result, Ok(()), "AlwaysValid should always succeed");
    }
}
