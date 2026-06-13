//! # edge-domain-valueobject
//!
//! The `ValueObject` port contract and `NonEmptyString` reference implementation.
//!
//! A value object has no identity — equality is by field value.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::NonEmptyString;
pub use saf::ValueObject;
pub use saf::ValueObjectError;
pub use saf::ValueObjectFactory;
pub use saf::non_empty_string;
pub use saf::VALUE_OBJECT_SVC;
