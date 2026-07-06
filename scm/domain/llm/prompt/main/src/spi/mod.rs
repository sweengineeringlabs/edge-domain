//! Extension hooks for downstream consumers.
//!
//! Downstream crates implement the `PromptBootstrap` contract from `crate::api`.
//! This crate has no pluggable strategy implementations beyond the reference
//! ones in `core/`; this module exists to satisfy the SEA structural layout.

// SPI extension anchor — satisfies the spi_dir_not_empty structural rule.
const _: () = ();
