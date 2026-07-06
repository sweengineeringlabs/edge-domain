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

pub use api::BootstrapNameRequest;
pub use api::BootstrapNameResponse;
pub use api::CompositePolicy;
pub use api::PolicyEvaluateRequest;
pub use api::PolicyNameRequest;
pub use api::PolicyNameResponse;
pub use api::PolicyError;
pub use api::StdPolicyFactory;
pub use saf::Policy;
pub use saf::PolicyBootstrap;
pub use saf::POLICY_BOOTSTRAP_SVC;
pub use saf::POLICY_BOOTSTRAP_SVC_FACTORY;
pub use saf::POLICY_SVC;
pub use saf::POLICY_SVC_FACTORY;
