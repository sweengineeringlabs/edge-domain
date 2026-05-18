//! Integration coverage for the domain validator layer.
//!
//! `Validator` and `validate()` are internal — the trait is an implementation
//! detail for domain-owned types only.  External consumers of edge-domain use
//! the handler + registry surface; they never call `validate()` directly.
//! Validation behaviour is covered by inline unit tests in `saf/validator.rs`.

/// Compile-time proof that the domain crate builds and exports its public API.
#[test]
fn test_domain_public_api_compiles() {
    use edge_domain::{Handler, HandlerError, HandlerRegistry, RequestContext};
    fn _assert_handler_registry_exists() {
        let _: HandlerRegistry<String, String> = HandlerRegistry::new();
    }
    let _ = (
        std::mem::size_of::<HandlerError>(),
        std::mem::size_of::<RequestContext>(),
    );
    fn _assert_object_safe(_: &dyn Handler<String, String>) {}
}
