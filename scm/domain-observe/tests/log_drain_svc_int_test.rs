use edge_domain_observe::{LogDrain, LogRecord, StdObserveFactory, LOG_DRAIN_SVC};

#[test]
fn test_noop_log_drain_svc_emits_info_record_happy() {
    let drain = StdObserveFactory::noop_log_drain();
    drain.emit(LogRecord::new("INFO", "svc.x", "started"));
}

#[test]
fn test_noop_log_drain_svc_emits_unknown_level_error() {
    let drain = StdObserveFactory::noop_log_drain();
    drain.emit(LogRecord::new("UNKNOWN", "svc.x", "unexpected level"));
}

#[test]
fn test_noop_log_drain_svc_emits_empty_fields_edge() {
    let drain = StdObserveFactory::noop_log_drain();
    drain.emit(LogRecord::new("", "", ""));
}

#[test]
fn test_log_drain_svc_key_namespaced_happy() {
    assert!(LOG_DRAIN_SVC.starts_with("edge."));
}

#[test]
fn test_log_drain_svc_returns_dyn_trait_object() {
    let _drain: Box<dyn LogDrain> = StdObserveFactory::noop_log_drain();
}
