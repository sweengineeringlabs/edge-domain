//! Query theme — neutral implementation marker types.

pub mod direct_query_bus;
pub mod noop_query;
pub mod std_query_bus_factory;

pub use direct_query_bus::DirectQueryBus;
pub use noop_query::NoopQuery;
pub use std_query_bus_factory::StdQueryBusFactory;
