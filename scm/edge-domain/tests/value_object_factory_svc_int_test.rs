//! Integration tests for the `ValueObjectFactory` SAF facade.

// This factory is only present when the inline valueobject fallback is compiled
// (i.e., when the `valueobject` feature is disabled).
#[cfg(not(feature = "valueobject"))]
mod tests {
    use edge_domain::{NonEmptyString, ValueObjectFactory};

    struct TestValueObjects;
    impl ValueObjectFactory for TestValueObjects {}

    /// @covers ValueObjectFactory::non_empty_string — happy path: valid string constructs Ok
    #[test]
    fn test_value_object_factory_non_empty_string_valid_returns_ok_happy() {
        let r = TestValueObjects::non_empty_string("hello".to_string());
        assert!(r.is_ok());
        assert_eq!(r.unwrap().as_str(), "hello");
    }

    /// @covers ValueObjectFactory::non_empty_string — error: empty string returns Err
    #[test]
    fn test_value_object_factory_non_empty_string_empty_returns_err_error() {
        let r = TestValueObjects::non_empty_string(String::new());
        assert!(r.is_err());
    }

    /// @covers ValueObjectFactory::non_empty_string — edge: whitespace-only is non-empty
    #[test]
    fn test_value_object_factory_non_empty_string_whitespace_is_non_empty_edge() {
        let r = TestValueObjects::non_empty_string("  ".to_string());
        assert!(r.is_ok(), "whitespace-only string is non-empty");
    }
}
