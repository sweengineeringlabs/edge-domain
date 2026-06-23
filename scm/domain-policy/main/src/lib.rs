//! # edge-domain-policy
//!
//! The `Policy` port contract — named, testable business rules with AND-composition.
//!
//! Distinct from `Validator`, which checks structural correctness. A policy evaluates
//! business intent against current domain state.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Policy;
pub use saf::PolicyBootstrap;
