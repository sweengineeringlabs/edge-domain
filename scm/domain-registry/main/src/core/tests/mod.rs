//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-domain-registry`'s only standalone public functions in `core/` are
//! [`InMemoryRegistry::new`](crate::api::InMemoryRegistry) and its `Default` impl, which are
//! exercised throughout the crate-root `tests/` directory (e.g. `tests/in_memory_registry_int_test.rs`,
//! `tests/registry_svc_int_test.rs`).
