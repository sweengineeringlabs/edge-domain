//! API layer — domain execution-unit contracts.
//!
//! Multi-theme layout (ADR-007): each theme owns its `traits/ types/ error/
//! vo/` subdirs. Cross-theme items live at the `api/` level in `traits/` and
//! `types/`. The `api/` surface is technology-neutral (ADR-008) — concrete
//! external-library implementations live under `spi/`.

#![allow(unused_imports)]

pub mod command;
pub mod domain;
pub mod event;
pub mod handler;
pub mod projection;
pub mod query;
pub mod repository;
pub mod saga;
pub mod service;
pub mod snapshot;
pub mod validator;
pub mod valueobject;
