//! # edge-domain-app
//!
//! The `Application` boot contract — Application and Bootstrap traits.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::AppError;
pub use api::Application;
pub use api::Bootstrap;
pub use api::NoopAppBootstrap;
pub use api::NoopApplication;
pub use saf::{APP_BOOTSTRAP_SVC, APP_BOOTSTRAP_SVC_FACTORY, APPLICATION_SVC, APPLICATION_SVC_FACTORY};
