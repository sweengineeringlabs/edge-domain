//! # edge-domain-security
//!
//! The `Security` port contract — caller identity and context enforcement.
//!
//! Use [`SecurityContext`] to carry principal identity through a request.
//! Use [`SecurityContextBuilder`] for ergonomic context construction.
//! Use [`NoopSecurity`] in tests or for open routes that require no guard.
//! Use [`SecurityServices`] or [`DEFAULT_SERVICES`] as a ready-made factory.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Principal;
pub use saf::Security;
pub use saf::SecurityBootstrap;
