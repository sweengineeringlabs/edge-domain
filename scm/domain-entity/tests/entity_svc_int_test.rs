//! Integration tests for the entity SAF facade.

use edge_domain_entity::{Entity, EntityError};

struct Point {
    id: u32,
}

impl Entity for Point {
    type Id = u32;
    fn id(&self) -> &u32 {
        &self.id
    }
}

/// @covers: Entity — SAF facade exports Entity
#[test]
fn test_entity_svc_entity_is_accessible_via_public_api_happy() {
    let p = Point { id: 7 };
    assert_eq!(*p.id(), 7u32);
}

/// @covers: EntityError — reserved error type is accessible via public API
#[test]
fn test_entity_svc_entity_error_is_accessible_via_public_api_edge() {
    // EntityError is #[non_exhaustive] empty enum — verify it is importable.
    // The match below is exhaustive (no variants) and never panics.
    fn check(_e: EntityError) {
        // #[non_exhaustive] empty enum — wildcard required from external crates; body is intentionally empty
        #[allow(clippy::match_single_binding)]
        match _e {
            _ => {}
        }
    }
    let _check = check; // bind to confirm function compiles

    // Verify EntityError type name is correct (edge case for empty enum)
    assert_eq!(std::any::type_name::<EntityError>(), "edge_domain_entity::EntityError");
}
