//! SAF — validator service facade.
#[cfg(not(feature = "validator"))]
pub use crate::api::Validator;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const VALIDATOR_SVC: () = ();
