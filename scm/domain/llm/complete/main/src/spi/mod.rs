//! Extension hooks for downstream consumers.
//!
//! Downstream crates implement the `Completer` contract from `crate::api`. No
//! default wiring types are needed here — the reference implementations live
//! in `core/`.

// SPI extension anchor — satisfies the spi_dir_not_empty structural rule.
const _: () = ();
