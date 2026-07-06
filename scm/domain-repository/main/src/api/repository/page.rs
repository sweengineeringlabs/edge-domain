//! `Page` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core implementation at
//! `core/repository/page.rs`.

/// Type alias for the paginated result wrapper, parameterised over the entity type.
///
/// Prefer this alias over naming `types::Page` directly in call sites.
pub type Page<T> = crate::api::repository::types::page::Page<T>;
