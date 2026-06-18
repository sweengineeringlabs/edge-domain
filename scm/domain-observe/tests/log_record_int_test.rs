use edge_domain_observe::LogRecord;

#[test]
fn test_log_record_new_happy_path_fields_preserved() {
    let r = LogRecord::new("INFO", "handler_a", "ok");
    assert_eq!(r.level, "INFO");
    assert_eq!(r.handler_id, "handler_a");
    assert_eq!(r.message, "ok");
}

#[test]
fn test_log_record_clone_is_equal() {
    let original = LogRecord::new("DEBUG", "h", "msg");
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_log_record_eq_different_level_not_equal() {
    let a = LogRecord::new("INFO", "h", "msg");
    let b = LogRecord::new("WARN", "h", "msg");
    assert_ne!(a, b);
}

#[test]
fn test_log_record_debug_does_not_panic() {
    let r = LogRecord::new("INFO", "h", "msg");
    let _ = format!("{r:?}");
}

#[test]
fn test_log_record_empty_fields_accepted() {
    let r = LogRecord::new("", "", "");
    assert_eq!(r.level, "");
    assert_eq!(r.handler_id, "");
    assert_eq!(r.message, "");
}
