//! [`NoopHandlerFactory`] — a no-op [`HandlerFactory`] structural placeholder.

/// A [`HandlerFactory`] that accepts a unit config and produces itself.
///
/// Used as a structural compliance anchor and test double.
/// The implementation lives in `core::handler::noop_handler_factory`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopHandlerFactory;
