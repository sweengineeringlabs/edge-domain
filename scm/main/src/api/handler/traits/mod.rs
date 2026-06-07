//! Handler port contracts — dispatch re-exports and domain-layer factory trait.

pub mod handler_factory;

pub use edge_dispatch::Handler;
pub use edge_dispatch::HandlerRegistry;
pub use handler_factory::HandlerFactory;
