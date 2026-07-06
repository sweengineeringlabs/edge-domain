//! Scaffold for external tests of standalone `pub fn` items declared directly in `saf/`.
//!
//! `edge-llm-reasoning`'s `saf/` layer has no standalone public functions reachable
//! outside their owning trait re-exports — all facade constructors are exercised
//! through the crate-root `tests/` directory (e.g. `tests/reasoning_bootstrap_e2e_test.rs`).
