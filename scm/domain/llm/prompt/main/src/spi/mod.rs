//! Extension hooks for downstream consumers.
//!
//! Downstream crates implement the `PromptFactory` contract from `crate::api`.
//! This module is the SEA SPI anchor — it signals extensibility without
//! surfacing internal types.

// SPI extension anchor — satisfies the spi_dir_not_empty structural rule.
const _: () = ();
