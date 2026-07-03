//! # edge-domain
//!
//! The L2 Domain contract — feature-gated port contracts.
//!
//! Activate individual features (`handler`, `event`, `command`, …) or `full`
//! to include all 14 port contracts. The default feature set enables `entity`
//! and `valueobject`.
//!
//! The domain layer has no outbound dependencies on infrastructure crates —
//! `edge-dispatch` and its decorator suite depend on this crate, not the reverse.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::*;
pub use saf::*;
