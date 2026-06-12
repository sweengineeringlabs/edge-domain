//! `CompositePolicy` — SEA Rule 121 api/core mirror.
//!
//! `CompositePolicy` is generic, so the structural auditor cannot match the
//! generic `impl Policy for CompositePolicy<I>` in `core/policy/composite_policy.rs`
//! to its api type by name. This path-level mirror provides the counterpart.
pub use crate::api::policy::types::CompositePolicy;
