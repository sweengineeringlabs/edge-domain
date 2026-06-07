//! # edge-domain
//!
//! The L2 Domain contract — business logic execution units.
//!
//! Defines the `Handler` port contract, `HandlerRegistry`, `RequestContext`,
//! and `HandlerError`. Concrete `Handler` implementations live in the
//! application built on top of this framework.
//!
//! The domain layer has no outbound dependencies on infrastructure crates —
//! `edge-dispatch` and its decorator suite depend on this crate, not the
//! reverse.

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
pub use api::handler::Handler;
pub use api::handler::HandlerError;
pub use api::handler::HandlerFactory;
pub use api::handler::HandlerRegistry;
pub use api::handler::RequestContext;
pub use api::handler::RequestContextBuilder;
pub use api::traits;
pub use api::types::NoopDomainExtension;
pub use gateway::*;
