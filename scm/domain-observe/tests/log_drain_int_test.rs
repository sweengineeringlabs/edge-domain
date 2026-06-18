use edge_domain_observe::{LogDrain, LogRecord, StdObserveFactory};

// --- emit ---

#[test]
fn test_emit_info_record_happy() {
    let drain = StdObserveFactory::noop_log_drain();
    let record = LogRecord::new("INFO", "handler_a", "request processed");
    drain.emit(record);
}

#[test]
fn test_emit_error_level_record_error() {
    let drain = StdObserveFactory::noop_log_drain();
    let record = LogRecord::new("ERROR", "handler_c", "unexpected failure");
    drain.emit(record);
}

#[test]
fn test_emit_empty_fields_edge() {
    let drain = StdObserveFactory::noop_log_drain();
    let record = LogRecord::new("", "", "");
    drain.emit(record);
}

#[test]
fn test_log_drain_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let drain = StdObserveFactory::noop_log_drain();
    assert_send_sync(&drain);
}

#[test]
fn test_log_drain_returns_dyn_trait_object() {
    let drain: Box<dyn LogDrain> = StdObserveFactory::noop_log_drain();
    drain.emit(LogRecord::new("INFO", "h", "msg"));
}
