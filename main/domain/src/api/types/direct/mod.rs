//! Direct (in-process) command and query bus types.

pub mod direct_command_bus;
pub mod direct_query_bus;

pub use direct_command_bus::DirectCommandBus;
pub use direct_query_bus::DirectQueryBus;
