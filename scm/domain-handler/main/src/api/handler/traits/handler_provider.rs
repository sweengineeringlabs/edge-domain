//! [`HandlerProvider`] — factory trait providing standard handler constructs.

use crate::api::handler::types::echo_handler::EchoHandler;
use crate::api::handler::types::in_process_handler_registry::InProcessHandlerRegistry;
use crate::api::handler::types::noop_handler_factory::NoopHandlerFactory;

/// Factory trait providing standard handler constructs without requiring
/// callers to name concrete types from `core/`.
pub trait HandlerProvider {
    /// Returns a stable, non-empty identifier for this provider.
    fn bootstrap_name(&self) -> &'static str {
        "handler_provider"
    }

    /// Construct an [`EchoHandler`] that reflects `String` requests back as responses.
    fn echo_handler(id: &str, pattern: &str) -> EchoHandler<String>
    where
        Self: Sized,
    {
        EchoHandler::new(id, pattern)
    }

    /// Construct a [`NoopHandlerFactory`] for use in tests and structural compliance.
    fn noop_handler_factory() -> NoopHandlerFactory
    where
        Self: Sized,
    {
        NoopHandlerFactory
    }

    /// Construct an [`InProcessHandlerRegistry`] for the given request/response types.
    fn in_process_registry<Req, Resp>() -> InProcessHandlerRegistry<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
        Self: Sized,
    {
        InProcessHandlerRegistry::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Prov;
    impl HandlerProvider for Prov {}

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_returns_nonempty_string_happy() {
        let p = Prov;
        assert!(!p.bootstrap_name().is_empty());
    }

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_is_deterministic_error() {
        let p = Prov;
        assert_eq!(p.bootstrap_name(), p.bootstrap_name());
    }

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_is_static_str_edge() {
        let p = Prov;
        let _name: &'static str = p.bootstrap_name();
    }

    #[test]
    fn test_echo_handler_creates_handler_with_id_happy() {
        let h = Prov::echo_handler("my-id", "/route");
        assert_eq!(h.id_str(), "my-id");
        assert_eq!(h.pattern_str(), "/route");
    }

    #[test]
    fn test_in_process_registry_creates_empty_registry_happy() {
        let reg = Prov::in_process_registry::<String, String>();
        assert_eq!(reg.handler_count(), 0);
    }

    #[test]
    fn test_noop_handler_factory_constructs_instance_edge() {
        let _f: NoopHandlerFactory = Prov::noop_handler_factory();
    }
}
