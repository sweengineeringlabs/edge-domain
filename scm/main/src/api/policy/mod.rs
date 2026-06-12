//! `Policy` theme — domain business-rule contracts.
//!
//! A [`Policy`] is a named, testable business rule evaluated synchronously
//! against domain state.  It is distinct from
//! [`Validator`](crate::api::validator::traits::Validator): a validator checks
//! structural correctness; a policy enforces business intent.
//!
//! # Public surface
//!
//! | Item | Kind | Description |
//! |------|------|-------------|
//! | [`Policy`] | trait | Single business rule contract |
//! | [`PolicyViolation`] | struct | Carries the violated rule name and reason |
//! | [`CompositePolicy`] | struct | AND-composition of multiple policies |

pub mod traits;
pub mod types;

pub use traits::{CompositePolicy, Policy};
pub use types::PolicyViolation;
