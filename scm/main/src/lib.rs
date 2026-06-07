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
pub use api::handler::EchoHandler;
pub use api::handler::EventEmittingHandler;
pub use api::handler::HandlerFactory;
pub use api::traits;
pub use api::types::NoopDomainExtension;
pub use gateway::*;
pub use saf::Handler;
pub use saf::HandlerError;
pub use saf::HandlerRegistry;
pub use saf::RequestContext;
pub use saf::RequestContextBuilder;
