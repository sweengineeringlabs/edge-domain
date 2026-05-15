//! Gateway — domain public entry point for external consumers.

pub(crate) mod input;
pub(crate) mod output;

pub use crate::api::traits::Validator;
pub use crate::saf::*;
