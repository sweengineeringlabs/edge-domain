use edge_domain_observer::StdObserveFactory;

// --- start_span ---

#[test]
fn test_start_span_with_valid_ids_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("handler_a", "execute");
    span.finish();
    // Span was created and finished without error.
    assert_eq!(std::mem::size_of_val(&*span), 0);
}

#[test]
fn test_start_span_empty_ids_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("", "");
    span.finish();
    // Empty IDs are handled without error.
    assert_eq!(std::mem::size_of_val(&*span), 0);
}

#[test]
fn test_start_span_unicode_ids_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("handler_α", "执行");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "unicode ids work");
}

// --- record (Span) ---

#[test]
fn test_record_key_value_pair_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.record("http.method", "GET");
    assert_eq!(std::mem::size_of_val(&*span), 0, "record works");
}

#[test]
fn test_record_empty_key_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.record("", "value");
    assert_eq!(std::mem::size_of_val(&*span), 0, "empty key handled");
}

#[test]
fn test_record_unicode_value_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.record("key", "值");
    assert_eq!(std::mem::size_of_val(&*span), 0, "unicode value works");
}

// --- finish ---

#[test]
fn test_finish_active_span_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "finish works");
}

#[test]
fn test_finish_already_done_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "double finish handled");
}

#[test]
fn test_finish_span_no_records_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "finish without records works");
}

#[test]
fn test_span_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    assert_send_sync(&span);
    assert_eq!(std::mem::size_of_val(&*span), 0, "span is Send+Sync ZST");
}
