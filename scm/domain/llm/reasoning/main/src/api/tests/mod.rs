//! Scaffold for external tests of standalone `pub fn` items declared directly in `api/`.
//!
//! `edge-llm-reasoning`'s `api/` layer has no standalone public functions reachable
//! outside their owning trait/type re-exports — all public surface is exercised
//! through the crate-root `tests/` directory (e.g. `tests/reasoning_svc_int_test.rs`).
