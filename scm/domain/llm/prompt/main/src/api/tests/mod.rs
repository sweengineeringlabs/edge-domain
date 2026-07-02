//! Scaffold for external tests of standalone `pub fn` items declared directly in `api/`.
//!
//! `edge-llm-prompt`'s `api/` layer has no standalone public functions reachable
//! outside their owning trait/type declarations — all public trait methods are
//! exercised through the crate-root `tests/` directory via the `saf/`-facade
//! re-exports (e.g. `tests/prompt_svc_int_test.rs`, `tests/context_manager_svc_int_test.rs`).
