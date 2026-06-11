//! `Validator` theme — default implementation of the
//! [`Validator`](crate::api::validator::traits::Validator) contract.

pub mod traits;
pub mod types;
pub mod validator_default;

pub use traits::Validator;
pub use types::ValidatorDefault;
