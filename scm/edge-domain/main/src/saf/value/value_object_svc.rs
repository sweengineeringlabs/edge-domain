//! SAF — value object service facade.
#[cfg(not(feature = "valueobject"))]
pub use crate::api::valueobject::NonEmptyString;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::valueobject::ValueObject;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::valueobject::ValueObjectError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const VALUE_OBJECT_CONTRACTS_SVC: () = ();
