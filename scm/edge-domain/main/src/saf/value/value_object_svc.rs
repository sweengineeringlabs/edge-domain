//! SAF — value object service facade.
#[cfg(not(feature = "valueobject"))]
pub use crate::api::NonEmptyString;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::ValueObject;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::ValueObjectError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const VALUE_OBJECT_CONTRACTS_SVC: () = ();
