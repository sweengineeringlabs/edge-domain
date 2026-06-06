//! SEA interface contract — crate-level cross-theme traits for `edge-domain`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Validator`] | Configuration validation contract (crate primary trait) |
//! | [`DomainExtension`] | Downstream extension hook (cross-cutting) |
//!
//! [`Validator`] is the crate's primary contract — consumed by the `validator`
//! theme's default implementation and by the SAF `validate_config` facade — so
//! it lives at the crate-level `traits/` surface.

pub mod domain_extension;
pub mod validator;

pub use domain_extension::DomainExtension;
pub use validator::Validator;
