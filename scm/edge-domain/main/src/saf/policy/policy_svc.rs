//! SAF — policy service facade.
#[cfg(not(feature = "policy"))]
pub use crate::api::CompositePolicy;
#[cfg(not(feature = "policy"))]
pub use crate::api::Policy;
#[cfg(not(feature = "policy"))]
pub use crate::api::PolicyViolation;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const POLICY_CONTRACTS_SVC: () = ();
