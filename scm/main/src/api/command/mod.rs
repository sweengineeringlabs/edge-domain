//! `Command` theme — write operations that mutate domain state.

pub mod error;
pub mod traits;
pub mod types;

pub use error::CommandError;
pub use traits::{Command, CommandBus};
pub use types::DirectCommandBus;
