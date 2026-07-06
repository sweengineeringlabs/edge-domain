//! `AlwaysMatchSpec` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core implementation at
//! `core/repository/always_match_spec.rs`.

/// Type alias for the null-object [`Spec`](crate::api::Spec) that matches every entity.
///
/// Prefer this alias over naming `types::AlwaysMatchSpec` directly in call sites.
pub type AlwaysMatchSpec<T> = crate::api::repository::types::always_match_spec::AlwaysMatchSpec<T>;
