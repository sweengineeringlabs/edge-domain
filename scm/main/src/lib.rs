//! # edge-domain
//!
//! The L2 Domain contract — business logic execution units.
//!
//! Defines the `Handler` trait and `HandlerRegistry`. Concrete `Handler`
//! implementations live in the application built on top of this framework.
//! The domain layer has no knowledge of ingress, proxy, or egress.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use api::event::{StageCompleted, StageFailed, StageSkipped, StageStarted};
pub use api::handler::EventEmittingHandler;
pub use api::traits;
pub use api::types::NoopDomainExtension;
pub use gateway::*;
