//! Extension hooks for downstream consumers.
//!
//! Downstream crates implement the `ProviderBootstrap` contract from `crate::api`.
//! This module re-exports the default handler wiring types so `saf/` can compose
//! them without importing from `core/` directly (SEA §7).

pub(crate) mod provider;

pub(crate) use provider::DefaultProviderHandler;

// SPI extension anchor — satisfies the spi_dir_not_empty structural rule.
const _: () = ();
