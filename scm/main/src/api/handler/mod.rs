//! `Handler` theme — dispatch primitives from `edge-dispatch` plus domain-layer decorators.

pub mod types;

pub use edge_dispatch::EchoHandler;
pub use edge_dispatch::Handler;
pub use edge_dispatch::HandlerError;
pub use edge_dispatch::HandlerRegistry;
pub use edge_dispatch::RequestContext;
pub use edge_dispatch::RequestContextBuilder;
pub use types::EventEmittingHandler;

pub mod error {
    //! Handler error types (re-exported from `edge-dispatch`).
    pub use edge_dispatch::HandlerError;
}

pub mod traits {
    //! Handler port contracts (re-exported from `edge-dispatch`).
    pub use edge_dispatch::Handler;
    pub use edge_dispatch::HandlerRegistry;
}

pub mod vo {
    //! Request-context value objects (re-exported from `edge-dispatch`).
    pub use edge_dispatch::RequestContext;
    pub use edge_dispatch::RequestContextBuilder;
}
