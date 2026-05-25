//! SEA interface contract — primary traits for `edge-domain`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Handler`] | Business logic execution unit |
//! | [`Validator`] | Configuration validation contract |

pub mod validator;

pub use validator::Validator;


