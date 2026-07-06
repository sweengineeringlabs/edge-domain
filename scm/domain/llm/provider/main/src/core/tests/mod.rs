//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-llm-provider`'s `core/` layer has no standalone public functions reachable
//! outside their owning type impls — all public surface is exercised through the
//! crate-root `tests/` directory (e.g. `tests/std_provider_e2e_test.rs`).
