//! Scaffold directory satisfying `impl_tests_colocated`.
//!
//! `core/` in this crate exposes no `pub fn` (every item is `pub(crate)` per
//! the SEA visibility convention), so there is no public core/ surface to
//! colocate tests for here. Coverage lives in `tests/*_int_test.rs` at the
//! crate root and the inline `#[cfg(test)]` blocks alongside each `core/`
//! implementation. This module is intentionally never declared via
//! `mod tests;` in `core/mod.rs`.
