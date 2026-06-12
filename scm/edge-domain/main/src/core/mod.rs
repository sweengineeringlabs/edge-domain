//! Core implementations.

#[cfg(not(feature = "clock"))]
pub(crate) mod clock;
pub(crate) mod command;
pub(crate) mod event;
pub(crate) mod handler;
#[cfg(not(feature = "policy"))]
pub(crate) mod policy;
pub(crate) mod projection;
pub(crate) mod query;
pub(crate) mod repository;
pub(crate) mod saga;
pub(crate) mod snapshot;
