//! # edge-domain-security
//!
//! The `Security` port contract — caller identity and context enforcement.
//!
//! Use [`SecurityContext`] to carry principal identity through a request.
//! Use [`NoopSecurity`] in tests or for open routes that require no guard.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::SecurityContext;
pub use api::SecurityError;
pub use saf::Principal;
pub use saf::Security;
pub use saf::SecurityBootstrap;
