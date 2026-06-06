//! `Handler` theme — re-exported from `edge-dispatch`.
//!
//! All dispatch primitives live in `edge-dispatch`. This module re-exports
//! them so existing `edge-domain` consumers do not need to change imports.

pub use edge_dispatch::EchoHandler;
pub use edge_dispatch::Handler;
pub use edge_dispatch::HandlerError;
pub use edge_dispatch::HandlerRegistry;
pub use edge_dispatch::RequestContext;
pub use edge_dispatch::RequestContextBuilder;

pub mod error {
    //! Handler error types (re-exported from `edge-dispatch`).
    pub use edge_dispatch::HandlerError;
}

pub mod traits {
    //! Handler port contracts (re-exported from `edge-dispatch`).
    pub use edge_dispatch::Handler;
    pub use edge_dispatch::HandlerRegistry;
}

pub mod types {
    //! Concrete handler types (re-exported from `edge-dispatch`).
    pub use edge_dispatch::EchoHandler;
    // The concrete HandlerRegistry struct — accessed via edge_dispatch directly.
    pub use edge_dispatch::HandlerRegistry as HandlerRegistryImpl;
}

pub mod vo {
    //! Request-context value objects (re-exported from `edge-dispatch`).
    pub use edge_dispatch::RequestContext;
    pub use edge_dispatch::RequestContextBuilder;
}
