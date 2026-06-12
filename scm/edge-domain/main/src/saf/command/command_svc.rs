//! SAF — command service facade.
#[cfg(not(feature = "command"))]
pub use crate::api::command::Command;
#[cfg(not(feature = "command"))]
pub use crate::api::command::CommandBus;
#[cfg(not(feature = "command"))]
pub use crate::api::command::CommandError;
#[cfg(not(feature = "command"))]
pub use crate::api::command::DirectCommandBus;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const COMMAND_SVC: () = ();
