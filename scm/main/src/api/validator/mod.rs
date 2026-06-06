//! `Validator` theme — default implementation of the crate-level
//! [`Validator`](crate::api::traits::Validator) contract.

pub mod types;

pub use crate::api::traits::Validator;
pub use types::ValidatorDefault;
