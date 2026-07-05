//! # edge-domain-clock
//!
//! The `Clock` port contract — injectable time source.
//!
//! Inject `SystemClock` in production and `FixedClock` in tests to make
//! time-dependent domain logic deterministic.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::BootstrapNameRequest;
pub use api::BootstrapNameResponse;
pub use api::ClockError;
pub use api::ElapsedSinceEpochRequest;
pub use api::ElapsedSinceEpochResponse;
pub use api::FixedClock;
pub use api::NowRequest;
pub use api::NowResponse;
pub use api::StdClockFactory;
pub use api::SystemClock;
pub use saf::Clock;
pub use saf::ClockBootstrap;
pub use saf::CLOCK_BOOTSTRAP_SVC;
pub use saf::CLOCK_BOOTSTRAP_SVC_FACTORY;
pub use saf::CLOCK_SVC;
pub use saf::CLOCK_SVC_FACTORY;
