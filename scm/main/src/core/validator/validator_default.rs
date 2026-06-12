//! [`Validator`] impl for [`ValidatorDefault`].

use crate::api::validator::traits::Validator;
use crate::api::validator::types::ValidatorDefault;

impl Validator for ValidatorDefault {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
