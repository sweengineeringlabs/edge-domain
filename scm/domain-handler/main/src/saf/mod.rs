mod handler;

// Public trait contracts only
pub use handler::{
    Handler, HandlerBootstrap, HandlerProvider, HandlerRegistry, HANDLER_BOOTSTRAP_SVC,
    HANDLER_PROVIDER_SVC, HANDLER_REGISTRY_SVC, HANDLER_SVC,
};

// Concrete types: internal use only
pub(crate) use handler::{
    EchoHandler, HandlerContext, HandlerError, InProcessHandlerRegistry, NoopHandlerFactory,
};
