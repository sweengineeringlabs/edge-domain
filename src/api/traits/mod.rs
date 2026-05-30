//! SEA interface contract — primary traits for `edge-domain`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Handler`] | Business logic execution unit |
//! | [`Validator`] | Configuration validation contract |

pub mod domain_extension;
pub mod validator;

pub use domain_extension::DomainExtension;
pub use validator::Validator;
