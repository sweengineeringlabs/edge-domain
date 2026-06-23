mod handler_bootstrap_svc;
mod handler_provider_svc;
mod handler_registry_svc;
mod handler_svc;

// Public trait contracts only
pub use handler_bootstrap_svc::{HandlerBootstrap, HANDLER_BOOTSTRAP_SVC};
pub use handler_provider_svc::{HandlerProvider, HANDLER_PROVIDER_SVC};
pub use handler_registry_svc::{HandlerRegistry, HANDLER_REGISTRY_SVC};
pub use handler_svc::{Handler, HANDLER_SVC};

// Concrete types: internal use only
pub(crate) use handler_bootstrap_svc::NoopHandlerFactory;
pub(crate) use handler_provider_svc::{EchoHandler, InProcessHandlerRegistry};
pub(crate) use handler_svc::{HandlerContext, HandlerError};
