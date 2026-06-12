//! SAF — validator service facade.

mod validator;

pub use crate::api::validator::AlwaysValid;
pub use crate::api::validator::Validator;
pub use crate::api::validator::ValidatorError;
pub use crate::api::validator::ValidatorFactory;
