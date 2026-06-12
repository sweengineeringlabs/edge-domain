//! SAF — clock service facade.
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::Clock;
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::FixedClock;
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::SystemClock;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const CLOCK_SVC: () = ();
