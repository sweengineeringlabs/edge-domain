//! [`HandlerProvider`] — factory trait providing standard handler constructs.

use crate::api::handler::types::echo_handler::EchoHandler;
use crate::api::handler::types::in_process_handler_registry::InProcessHandlerRegistry;
use crate::api::handler::types::noop_handler_factory::NoopHandlerFactory;
use crate::api::handler::types::std_registry_bridge::StdRegistryBridge;

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
        EchoHandler::from((id, pattern))
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
        InProcessHandlerRegistry::default()
    }

    /// Construct the standard [`StdRegistryBridge`] for bridging service registries into handler registries.
    fn default_bridge() -> StdRegistryBridge
    where
        Self: Sized,
    {
        StdRegistryBridge
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler::traits::handler_bootstrap::HandlerBootstrap;

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
        assert_eq!(p.bootstrap_name(), "handler_provider");
    }

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_is_static_str_edge() {
        let p = Prov;
        let name: &'static str = p.bootstrap_name();
        assert_eq!(name, "handler_provider");
    }

    #[test]
    fn test_echo_handler_creates_handler_with_id_happy() {
        let h = Prov::echo_handler("my-id", "/route");
        assert_eq!(h.id, "my-id");
        assert_eq!(h.pattern, "/route");
    }

    #[test]
    fn test_in_process_registry_creates_empty_registry_happy() {
        use crate::api::handler::traits::handler_registry::HandlerRegistry;
        let reg = Prov::in_process_registry::<String, String>();
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn test_noop_handler_factory_constructs_instance_edge() {
        let f: NoopHandlerFactory = Prov::noop_handler_factory();
        assert_eq!(f.bootstrap_name(), "handler");
    }

    /// @covers: default_bridge
    #[test]
    fn test_default_bridge_constructs_std_registry_bridge_happy() {
        let b: StdRegistryBridge = Prov::default_bridge();
        assert_eq!(format!("{b:?}"), "StdRegistryBridge");
    }

    /// @covers: default_bridge
    #[test]
    fn test_default_bridge_bridge_is_zero_sized_error() {
        assert_eq!(std::mem::size_of::<StdRegistryBridge>(), 0);
    }

    /// @covers: default_bridge
    #[test]
    fn test_default_bridge_is_copy_edge() {
        let b = Prov::default_bridge();
        let c = b; // copy — b still usable
        assert_eq!(b, c);
    }
}
