//! # edge-llm-provider
//!
//! LLM Provider domain primitive (ADR-033): a pluggable execution-backend
//! abstraction for swappable LLM providers (OpenAI, Claude, local models).
//!
//! Public surface is delegated entirely through `saf/`.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::{
    ExecutionModel, Provider, ProviderBootstrap, StreamHandler, EXECUTION_MODEL_SVC,
    PROVIDER_BOOTSTRAP_SVC, PROVIDER_COMPLETER_SVC, PROVIDER_SVC, STREAM_HANDLER_SVC,
};
