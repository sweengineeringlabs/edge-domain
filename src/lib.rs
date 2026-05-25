//! # edge-domain
//!
//! The L2 Domain contract — business logic execution units.
//!
//! Defines the `Handler` trait and `HandlerRegistry`. Concrete `Handler`
//! implementations live in the application built on top of this framework.
//! The domain layer has no knowledge of ingress, proxy, or egress.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use saf::*;
pub use api::traits;
