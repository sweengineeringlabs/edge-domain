//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-llm-prompt`'s `core/` layer has no standalone public functions reachable
//! outside their owning `impl` blocks — all public constructors are exercised
//! through the crate-root `tests/` directory via the `api/` re-exports that
//! wrap them (e.g. `tests/context_manager_e2e_test.rs`, `tests/prompt_e2e_test.rs`).
