//! SAF — policy service facade.
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::CompositePolicy;
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::Policy;
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::PolicyViolation;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const POLICY_CONTRACTS_SVC: () = ();
