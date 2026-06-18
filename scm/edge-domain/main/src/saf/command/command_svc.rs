//! SAF — command service facade.
#[cfg(not(feature = "command"))]
pub use crate::api::Command;
#[cfg(not(feature = "command"))]
pub use crate::api::CommandBus;
#[cfg(not(feature = "command"))]
pub use crate::api::CommandError;
#[cfg(not(feature = "command"))]
pub use crate::api::DirectCommandBus;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const COMMAND_SVC: () = ();
