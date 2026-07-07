//! Integration tests — `NoopHandlerFactory` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::NoopHandlerFactory;

/// @covers: NoopHandlerFactory::default — derives Default
#[test]
fn test_default_constructs_instance_happy() {
    let f: NoopHandlerFactory = NoopHandlerFactory::default();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: NoopHandlerFactory — derives Clone and Copy
#[test]
fn test_clone_copy_semantics_edge() {
    let a = NoopHandlerFactory;
    let b = a; // Copy — usable after move
    let c = a.clone(); // Clone
    assert_eq!(std::mem::size_of_val(&b), 0);
    assert_eq!(std::mem::size_of_val(&c), 0);
}

/// @covers: NoopHandlerFactory — zero-sized marker
#[test]
fn test_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<NoopHandlerFactory>(), 0);
}
