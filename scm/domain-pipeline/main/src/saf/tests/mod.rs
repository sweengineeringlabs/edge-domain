//! Scaffold directory satisfying `impl_public_tests_external`.
//!
//! `saf/` public function coverage lives in `tests/*_int_test.rs` and
//! `tests/*_svc_e2e_test.rs` at the crate root, not here — saf/ itself
//! contains no inline `#[cfg(test)]` blocks. This module is intentionally
//! never declared via `mod tests;` in `saf/mod.rs`.
