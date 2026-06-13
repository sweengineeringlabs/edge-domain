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

pub use saf::Clock;
pub use saf::ClockError;
pub use saf::ClockFactory;
pub use saf::StdClockFactory;
pub use saf::FixedClock;
pub use saf::SystemClock;
