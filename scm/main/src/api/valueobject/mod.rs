//! `ValueObject` theme — the [`ValueObject`] marker trait contract.
//!
//! This module owns the trait definition and a single reference implementation
//! ([`NonEmptyString`]).  Domain-specific value objects (`OrderId`, `Money`,
//! etc.) belong in `api/<theme>/types/` of their owning theme and implement
//! [`ValueObject`] there.

pub mod traits;
pub mod types;

pub use traits::ValueObject;
pub use types::NonEmptyString;
