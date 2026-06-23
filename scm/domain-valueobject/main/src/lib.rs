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

pub use saf::ValueObject;
pub use saf::ValueObjectBootstrap;
pub use saf::VALUE_OBJECT_FACTORY_SVC;
pub use saf::VALUE_OBJECT_SVC;
