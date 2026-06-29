//! Colocated tests for core/ public functions.
//!
//! Guarded by `#[cfg(test)] mod tests;` in `core/mod.rs` — no test code
//! reaches the production build.
mod pem_tls_config_test;
