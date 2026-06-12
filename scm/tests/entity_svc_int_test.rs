//! Integration tests for `Entity` trait.

use edge_domain::Entity;

struct OrderLine {
    id: u64,
    #[allow(dead_code)]
    quantity: u32,
}

impl Entity for OrderLine {
    type Id = u64;
    fn id(&self) -> &u64 {
        &self.id
    }
}

/// @covers: Entity::id
#[test]
fn test_id_returns_assigned_id_happy() {
    let line = OrderLine {
        id: 42,
        quantity: 3,
    };
    assert_eq!(*line.id(), 42u64);
}

/// @covers: Entity::id — two instances with different IDs are distinct
#[test]
fn test_id_two_instances_with_different_ids_are_not_equal_error() {
    let a = OrderLine { id: 1, quantity: 5 };
    let b = OrderLine { id: 2, quantity: 5 };
    assert_ne!(a.id(), b.id());
}

/// @covers: Entity::id — works with a non-numeric Id type
#[test]
fn test_id_with_string_id_type_returns_correct_value_edge() {
    struct Tag {
        id: String,
    }
    impl Entity for Tag {
        type Id = String;
        fn id(&self) -> &String {
            &self.id
        }
    }
    let tag = Tag {
        id: "label-a".to_string(),
    };
    assert_eq!(tag.id(), "label-a");
}
