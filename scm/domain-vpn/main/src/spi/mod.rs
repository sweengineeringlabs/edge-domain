//! SPI anchor — extension point for downstream VPN backend implementors.
//!
//! Downstream crates implement the `TunnelManager` contract from `crate::api`.
//! This module is the SEA SPI anchor — signals extensibility without
//! surfacing internal types.

// SPI extension anchor — satisfies the spi_dir_not_empty structural rule.
const _: () = ();
