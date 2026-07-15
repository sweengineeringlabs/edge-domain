//! Umbrella-level integration tests that exercise `edge-domain-entity` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_entity::{Entity, EntityError, IdRequest, IdResponse};

struct Invoice {
    id: u64,
}

impl Entity for Invoice {
    type Id = u64;
    fn id(&self, _req: IdRequest) -> Result<IdResponse<'_, u64>, EntityError> {
        Ok(IdResponse { id: &self.id })
    }
}

/// @covers: edge-domain-entity::Entity — accessible as a direct dep of the umbrella
#[test]
fn test_entity_sub_crate_accessible_as_umbrella_dep_happy() {
    let inv = Invoice { id: 99 };
    assert_eq!(*inv.id(IdRequest).unwrap().id, 99u64);
}

/// @covers: edge-domain-entity::Entity — two entities with different IDs are not equal (identity semantics)
#[test]
fn test_entity_sub_crate_different_ids_are_distinct_error() {
    let a = Invoice { id: 1 };
    let b = Invoice { id: 2 };
    assert_ne!(a.id(IdRequest).unwrap().id, b.id(IdRequest).unwrap().id);
}
