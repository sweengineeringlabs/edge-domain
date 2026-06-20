//! End-to-end usage example for `edge-domain-observe`.
//!
//! Demonstrates the full observability triplet using noop implementations —
//! the same API contract that production SDK adapters satisfy.

use edge_domain_observe::{LogRecord, ObserveBootstrap, StdObserveFactory};

fn main() {
    // --- Tracer ---
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("order_handler", "execute");
    span.record("order.id", "ord-42");
    span.record("order.items", "3");
    span.finish();

    // --- Metrics ---
    let registry = StdObserveFactory::noop_metric_registry();
    let req_counter = registry.counter("handler.requests.total");
    let latency = registry.histogram("handler.latency_ms");
    let queue = registry.gauge("handler.queue_depth");

    req_counter.increment(1);
    latency.record(12.5);
    queue.set(4.0);

    // --- Log drain ---
    let drain = StdObserveFactory::noop_log_drain();
    drain.emit(LogRecord::new(
        "INFO",
        "order_handler",
        "order processed successfully",
    ));

    // --- Factory approach ---
    let factory = StdObserveFactory::create_factory();
    assert!(factory.validate().is_ok());

    let factory2 = StdObserveFactory::std_factory();
    let _tracer2 = factory2.build_handler_tracer();
    let _registry2 = factory2.build_metric_registry();
    let _drain2 = factory2.build_log_drain();

    println!("observe example complete — all primitives exercised");
}
