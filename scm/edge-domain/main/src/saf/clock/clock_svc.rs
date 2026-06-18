//! SAF — clock service facade.
#[cfg(not(feature = "clock"))]
pub use crate::api::Clock;
#[cfg(not(feature = "clock"))]
pub use crate::api::FixedClock;
#[cfg(not(feature = "clock"))]
pub use crate::api::SystemClock;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const CLOCK_SVC: () = ();
