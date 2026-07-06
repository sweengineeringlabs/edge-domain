//! Scaffold for external tests of standalone `pub fn` items declared directly in `saf/`.
//!
//! `edge-llm-agent`'s `saf/` layer has no standalone public functions reachable
//! outside their owning trait/const re-exports — all public surface is exercised
//! through the crate-root `tests/` directory.
