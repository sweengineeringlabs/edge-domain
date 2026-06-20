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

pub use saf::CompositePolicy;
pub use saf::Policy;
pub use saf::PolicyBootstrap;
pub use saf::PolicyViolation;
pub use saf::StdPolicyFactory;
pub use saf::POLICY_SVC;
pub use saf::POLICY_FACTORY_SVC;
