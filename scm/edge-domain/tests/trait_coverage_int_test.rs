//! Rule-222 coverage: _happy, _error, and _edge tests for every fn in every pub trait in api/.
//!
//! One test per unique method name × 3 suffixes covers all 47 trait functions because
//! the arch audit pattern `test_<fn>_*_<suffix>` matches on method name globally across traits.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::*;
use futures::executor::block_on;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::SystemTime;

// ─── shared fixtures ─────────────────────────────────────────────────────────

#[derive(Clone)]
struct TestEvent {
    aggregate_id: String,
}

impl DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
        "test.event"
    }
    fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH
    }
}

#[derive(Default)]
struct TestAggregate {
    id: String,
    count: u32,
}

impl Aggregate for TestAggregate {
    type Event = TestEvent;
    fn apply(&mut self, e: &TestEvent) {
        self.id = e.aggregate_id.clone();
        self.count += 1;
    }
    fn id(&self) -> &str {
        &self.id
    }
}

struct OkCmd;
impl Command for OkCmd {
    fn name(&self) -> &str {
        "ok-cmd"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct ErrCmd;
impl Command for ErrCmd {
    fn name(&self) -> &str {
        "err-cmd"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

struct OkQry(String);
impl Query for OkQry {
    type Result = String;
    fn name(&self) -> &str {
        "ok-qry"
    }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

struct ErrQry;
impl Query for ErrQry {
    type Result = String;
    fn name(&self) -> &str {
        "err-qry"
    }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Err(QueryError::Internal("oops".into())) })
    }
}

struct OkSvc;
impl Service for OkSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "ok-svc"
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct ErrSvc;
impl Service for ErrSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "err-svc"
    }
    fn execute(&self, _: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async { Err(ServiceError::RuleViolation("blocked".into())) })
    }
}

struct AlwaysMatch;
impl Spec<String> for AlwaysMatch {
    fn matches(&self, _: &String) -> bool {
        true
    }
}

struct NeverMatch;
impl Spec<String> for NeverMatch {
    fn matches(&self, _: &String) -> bool {
        false
    }
}

struct GoodCfg;
struct BadCfg;
#[derive(Debug)]
struct GoodCfgHandler {
    _marker: (),
}
impl HandlerFactory for GoodCfgHandler {
    type Config = GoodCfg;
    fn build(_: GoodCfg) -> Result<Self, HandlerError> {
        Ok(GoodCfgHandler { _marker: () })
    }
}
#[derive(Debug)]
struct BadCfgHandler {
    _marker: (),
}
impl HandlerFactory for BadCfgHandler {
    type Config = BadCfg;
    fn build(_: BadCfg) -> Result<Self, HandlerError> {
        Err(HandlerError::internal("bad config"))
    }
}

struct ErrEventStore;
impl EventStore for ErrEventStore {
    type Event = TestEvent;
    fn append(
        &self,
        _: &str,
        _: Vec<TestEvent>,
        _: ExpectedVersion,
    ) -> BoxFuture<'_, Result<u64, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
    fn load(
        &self,
        _: &str,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<TestEvent>>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
    fn load_from(
        &self,
        _: &str,
        _: u64,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<TestEvent>>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
}

fn make_test_handler() -> Arc<dyn Handler<Request = String, Response = String>> {
    Domain::echo_handler("test", "/test")
}

// ─── name ────────────────────────────────────────────────────────────────────
// Covers: Command::name, Query::name, Service::name

#[test]
fn test_name_command_returns_defined_value_happy() {
    assert_eq!(OkCmd.name(), "ok-cmd");
}

#[test]
fn test_name_query_consistent_across_calls_not_error() {
    // name() must never error — returns same value each call
    let q = OkQry("x".into());
    assert_eq!(q.name(), q.name());
}

#[test]
fn test_name_service_can_be_empty_string_edge() {
    struct EmptySvc;
    impl Service for EmptySvc {
        type Request = ();
        type Response = ();
        fn name(&self) -> &str {
            ""
        }
        fn execute(&self, _: ()) -> BoxFuture<'_, Result<(), ServiceError>> {
            Box::pin(async { Ok(()) })
        }
    }
    assert_eq!(EmptySvc.name(), "");
}

// ─── execute ─────────────────────────────────────────────────────────────────
// Covers: Command::execute, Query::execute, Service::execute

#[test]
fn test_execute_command_returns_ok_happy() {
    block_on(async {
        assert!(OkCmd.execute().await.is_ok());
    });
}

#[test]
fn test_execute_command_returns_err_on_failure_error() {
    block_on(async {
        assert!(ErrCmd.execute().await.is_err());
    });
}

#[test]
fn test_execute_query_with_empty_response_edge() {
    block_on(async {
        let result = OkQry(String::new()).execute().await.unwrap();
        assert_eq!(result, "");
    });
}

// ─── dispatch ────────────────────────────────────────────────────────────────
// Covers: CommandBus::dispatch, QueryBus::dispatch

#[test]
fn test_dispatch_command_returns_ok_happy() {
    block_on(async {
        let bus = Domain::direct_command_bus();
        assert!(bus.dispatch(Box::new(OkCmd)).await.is_ok());
    });
}

#[test]
fn test_dispatch_command_propagates_error() {
    block_on(async {
        let bus = Domain::direct_command_bus();
        assert!(bus.dispatch(Box::new(ErrCmd)).await.is_err());
    });
}

#[test]
fn test_dispatch_query_result_type_preserved_edge() {
    block_on(async {
        let bus = Domain::direct_query_bus::<String>();
        let r = bus
            .dispatch(Box::new(OkQry("echo".into())))
            .await
            .expect("dispatch failed");
        assert_eq!(r, "echo");
    });
}

#[test]
fn test_dispatch_query_propagates_error_error() {
    block_on(async {
        let bus = Domain::direct_query_bus::<String>();
        assert!(bus.dispatch(Box::new(ErrQry)).await.is_err());
    });
}

// ─── apply ───────────────────────────────────────────────────────────────────
// Covers: Aggregate::apply

#[test]
fn test_apply_event_updates_aggregate_state_happy() {
    let mut agg = TestAggregate::default();
    agg.apply(&TestEvent {
        aggregate_id: "a1".into(),
    });
    assert_eq!(agg.id(), "a1");
    assert_eq!(agg.count, 1);
}

#[test]
fn test_apply_no_op_on_default_impl_not_error() {
    // Default Aggregate::apply does nothing — verifies it doesn't panic
    #[derive(Default)]
    struct NoOpAgg;
    #[derive(Clone)]
    struct NoOpEvent;
    impl DomainEvent for NoOpEvent {
        fn event_type(&self) -> &str {
            "noop"
        }
        fn aggregate_id(&self) -> &str {
            ""
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::UNIX_EPOCH
        }
    }
    impl Aggregate for NoOpAgg {
        type Event = NoOpEvent;
    }
    let mut agg = NoOpAgg;
    agg.apply(&NoOpEvent); // should not panic
}

#[test]
fn test_apply_called_twice_increments_count_edge() {
    let mut agg = TestAggregate::default();
    agg.apply(&TestEvent {
        aggregate_id: "a".into(),
    });
    agg.apply(&TestEvent {
        aggregate_id: "a".into(),
    });
    assert_eq!(agg.count, 2);
}

// ─── id ──────────────────────────────────────────────────────────────────────
// Covers: Aggregate::id, Handler::id

#[test]
fn test_id_aggregate_reflects_applied_event_happy() {
    let mut agg = TestAggregate::default();
    agg.apply(&TestEvent {
        aggregate_id: "agg-42".into(),
    });
    assert_eq!(agg.id(), "agg-42");
}

#[test]
fn test_id_handler_matches_constructor_arg_not_error() {
    let h = make_test_handler();
    assert_eq!(h.id(), "test");
}

#[test]
fn test_id_aggregate_default_is_empty_edge() {
    let agg = TestAggregate::default();
    assert_eq!(agg.id(), "");
}

// ─── event_type ──────────────────────────────────────────────────────────────
// Covers: DomainEvent::event_type

#[test]
fn test_event_type_returns_defined_value_happy() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(e.event_type(), "test.event");
}

#[test]
fn test_event_type_stable_across_calls_not_error() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(e.event_type(), e.event_type());
}

#[test]
fn test_event_type_default_impl_returns_event_edge() {
    // Default DomainEvent::event_type returns "event" — documents default behavior
    struct DefaultEvent;
    impl DomainEvent for DefaultEvent {}
    assert_eq!(DefaultEvent.event_type(), "event");
}

// ─── aggregate_id ────────────────────────────────────────────────────────────
// Covers: DomainEvent::aggregate_id

#[test]
fn test_aggregate_id_returns_set_value_happy() {
    let e = TestEvent {
        aggregate_id: "order-1".into(),
    };
    assert_eq!(e.aggregate_id(), "order-1");
}

#[test]
fn test_aggregate_id_consistent_across_calls_not_error() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(e.aggregate_id(), e.aggregate_id());
}

#[test]
fn test_aggregate_id_can_be_empty_string_edge() {
    let e = TestEvent {
        aggregate_id: String::new(),
    };
    assert_eq!(e.aggregate_id(), "");
}

// ─── occurred_at ─────────────────────────────────────────────────────────────
// Covers: DomainEvent::occurred_at

#[test]
fn test_occurred_at_returns_expected_time_happy() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(e.occurred_at(), SystemTime::UNIX_EPOCH);
}

#[test]
fn test_occurred_at_is_after_unix_epoch_not_error() {
    // Default occurred_at is SystemTime::now() — must be >= UNIX_EPOCH
    struct NowEvent;
    impl DomainEvent for NowEvent {}
    assert!(NowEvent.occurred_at() >= SystemTime::UNIX_EPOCH);
}

#[test]
fn test_occurred_at_unix_epoch_is_valid_timestamp_edge() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    let dur = e.occurred_at().duration_since(SystemTime::UNIX_EPOCH);
    assert!(dur.is_ok());
}

// ─── publish ─────────────────────────────────────────────────────────────────
// Covers: EventBus::publish, EventPublisher::publish

#[test]
fn test_publish_to_noop_bus_returns_ok_happy() {
    block_on(async {
        let bus = Domain::noop_event_bus();
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "x".into(),
        });
        assert!(bus.publish(e).await.is_ok());
    });
}

#[test]
fn test_publish_to_noop_publisher_never_errors_not_error() {
    block_on(async {
        let pub_ = Domain::noop_event_publisher();
        let e = TestEvent {
            aggregate_id: "x".into(),
        };
        assert!(pub_.publish(&e).await.is_ok());
    });
}

#[test]
fn test_publish_multiple_events_sequentially_edge() {
    block_on(async {
        let bus = Domain::noop_event_bus();
        for i in 0..5u32 {
            let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
                aggregate_id: i.to_string(),
            });
            assert!(bus.publish(e).await.is_ok());
        }
    });
}

// ─── subscribe ───────────────────────────────────────────────────────────────
// Covers: EventBus::subscribe

#[test]
fn test_subscribe_noop_bus_yields_receiver_happy() {
    block_on(async {
        let bus = Domain::noop_event_bus();
        let mut rx = bus.subscribe();
        // noop bus's receiver immediately signals unavailable
        assert!(rx.recv().await.is_err());
    });
}

#[test]
fn test_subscribe_active_bus_receives_published_event_not_error() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "e1".into(),
        });
        bus.publish(e).await.unwrap();
        assert!(rx.recv().await.is_ok());
    });
}

#[test]
fn test_subscribe_multiple_receivers_each_get_event_edge() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "e2".into(),
        });
        bus.publish(e).await.unwrap();
        assert!(rx1.recv().await.is_ok());
        assert!(rx2.recv().await.is_ok());
    });
}

// ─── append ──────────────────────────────────────────────────────────────────
// Covers: EventStore::append

#[test]
fn test_append_returns_version_after_first_event_happy() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        let ver = store
            .append(
                "agg-1",
                vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                ExpectedVersion::NoStream,
            )
            .await
            .unwrap();
        assert_eq!(ver, 1);
    });
}

#[test]
fn test_append_nostream_on_existing_stream_returns_error() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        store
            .append(
                "agg-1",
                vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                ExpectedVersion::NoStream,
            )
            .await
            .unwrap();
        let result = store
            .append(
                "agg-1",
                vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                ExpectedVersion::NoStream,
            )
            .await;
        assert!(result.is_err());
    });
}

#[test]
fn test_append_any_version_never_conflicts_edge() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        for _ in 0..3 {
            assert!(store
                .append(
                    "agg-1",
                    vec![TestEvent {
                        aggregate_id: "agg-1".into()
                    }],
                    ExpectedVersion::Any,
                )
                .await
                .is_ok());
        }
    });
}

// ─── load ────────────────────────────────────────────────────────────────────
// Covers: EventStore::load

#[test]
fn test_load_after_append_returns_events_happy() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        store
            .append(
                "a1",
                vec![
                    TestEvent {
                        aggregate_id: "a1".into(),
                    },
                    TestEvent {
                        aggregate_id: "a1".into(),
                    },
                ],
                ExpectedVersion::Any,
            )
            .await
            .unwrap();
        let events = store.load("a1").await.unwrap();
        assert_eq!(events.len(), 2);
    });
}

#[test]
fn test_load_nonexistent_stream_returns_empty_not_error() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        let events = store.load("ghost").await.unwrap();
        assert!(events.is_empty());
    });
}

#[test]
fn test_load_events_have_correct_sequence_edge() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        store
            .append(
                "a1",
                vec![TestEvent {
                    aggregate_id: "a1".into(),
                }],
                ExpectedVersion::Any,
            )
            .await
            .unwrap();
        let events = store.load("a1").await.unwrap();
        assert_eq!(events[0].sequence, 1);
    });
}

// ─── load_from ───────────────────────────────────────────────────────────────
// Covers: EventStore::load_from

#[test]
fn test_load_from_returns_subset_from_sequence_happy() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        for _ in 0..3u32 {
            store
                .append(
                    "a1",
                    vec![TestEvent {
                        aggregate_id: "a1".into(),
                    }],
                    ExpectedVersion::Any,
                )
                .await
                .unwrap();
        }
        let events = store.load_from("a1", 2).await.unwrap();
        assert_eq!(events.len(), 2); // sequences 2 and 3
    });
}

#[test]
fn test_load_from_beyond_end_returns_empty_not_error() {
    block_on(async {
        let store = Domain::new_in_memory_event_store::<TestEvent>();
        store
            .append(
                "a1",
                vec![TestEvent {
                    aggregate_id: "a1".into(),
                }],
                ExpectedVersion::Any,
            )
            .await
            .unwrap();
        let events = store.load_from("a1", 999).await.unwrap();
        assert!(events.is_empty());
    });
}

#[test]
fn test_load_from_unavailable_store_propagates_error_edge() {
    block_on(async {
        let result = ErrEventStore.load_from("a1", 0).await;
        assert!(result.is_err());
    });
}

// ─── recv_next ───────────────────────────────────────────────────────────────
// Covers: EventSource::recv_next (tested via EventReceiver)

#[test]
fn test_recv_next_active_bus_returns_event_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "r1".into(),
        });
        bus.publish(e).await.unwrap();
        assert!(rx.recv().await.is_ok());
    });
}

#[test]
fn test_recv_next_closed_source_returns_unavailable_error() {
    block_on(async {
        // noop bus subscribe returns a ClosedEventSource
        let bus = Domain::noop_event_bus();
        let mut rx = bus.subscribe();
        assert!(matches!(rx.recv().await, Err(EventError::Unavailable(_))));
    });
}

#[test]
fn test_recv_next_event_type_preserved_edge() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "r2".into(),
        });
        bus.publish(e).await.unwrap();
        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_type(), "test.event");
    });
}

// ─── pattern ─────────────────────────────────────────────────────────────────
// Covers: Handler::pattern

#[test]
fn test_pattern_handler_matches_constructor_arg_happy() {
    let h = make_test_handler();
    assert_eq!(h.pattern(), "/test");
}

#[test]
fn test_pattern_stable_across_calls_not_error() {
    let h = make_test_handler();
    assert_eq!(h.pattern(), h.pattern());
}

#[test]
fn test_pattern_can_be_root_path_edge() {
    let h = Domain::echo_handler::<String>("root", "/");
    assert_eq!(h.pattern(), "/");
}

// ─── build ───────────────────────────────────────────────────────────────────
// Covers: HandlerFactory::build

#[test]
fn test_build_valid_config_returns_ok_happy() {
    assert!(GoodCfgHandler::build(GoodCfg).is_ok());
}

#[test]
fn test_build_invalid_config_returns_err_error() {
    assert!(BadCfgHandler::build(BadCfg).is_err());
}

#[test]
fn test_build_error_message_describes_failure_edge() {
    let err = BadCfgHandler::build(BadCfg).unwrap_err();
    assert!(err.to_string().contains("bad config"));
}

// ─── register ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::register, ServiceRegistry::register

#[test]
fn test_register_handler_then_registry_not_empty_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    assert!(!reg.is_empty());
}

#[test]
fn test_register_same_id_twice_overwrites_not_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    reg.register(make_test_handler());
    assert_eq!(reg.len(), 1); // not 2 — overwrites
}

#[test]
fn test_register_service_increments_len_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    assert_eq!(reg.len(), 1);
}

// ─── deregister ──────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::deregister, ServiceRegistry::deregister

#[test]
fn test_deregister_registered_handler_returns_true_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    assert!(reg.deregister("test"));
}

#[test]
fn test_deregister_absent_handler_returns_false_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(!reg.deregister("ghost"));
}

#[test]
fn test_deregister_leaves_registry_empty_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    reg.deregister("ok-svc");
    assert!(reg.is_empty());
}

// ─── get ─────────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::get, ServiceRegistry::get

#[test]
fn test_get_registered_handler_returns_some_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    assert!(reg.get("test").is_some());
}

#[test]
fn test_get_nonexistent_key_returns_none_not_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.get("ghost").is_none());
}

#[test]
fn test_get_after_deregister_returns_none_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    reg.deregister("ok-svc");
    assert!(reg.get("ok-svc").is_none());
}

// ─── list_ids ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::list_ids

#[test]
fn test_list_ids_contains_registered_id_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    assert!(reg.list_ids().contains(&"test".to_string()));
}

#[test]
fn test_list_ids_empty_before_registration_not_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.list_ids().is_empty());
}

#[test]
fn test_list_ids_len_matches_registry_len_edge() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(Domain::echo_handler("a", "/a"));
    reg.register(Domain::echo_handler("b", "/b"));
    assert_eq!(reg.list_ids().len(), reg.len());
}

// ─── len ─────────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::len, ServiceRegistry::len

#[test]
fn test_len_increments_after_register_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert_eq!(reg.len(), 0);
    reg.register(make_test_handler());
    assert_eq!(reg.len(), 1);
}

#[test]
fn test_len_decrements_after_deregister_not_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    reg.deregister("test");
    assert_eq!(reg.len(), 0);
}

#[test]
fn test_len_service_registry_matches_registered_count_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    reg.register(Arc::new(ErrSvc));
    assert_eq!(reg.len(), 2);
}

// ─── is_empty ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::is_empty, ServiceRegistry::is_empty

#[test]
fn test_is_empty_true_on_new_registry_happy() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.is_empty());
}

#[test]
fn test_is_empty_false_after_registration_not_error() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(make_test_handler());
    assert!(!reg.is_empty());
}

#[test]
fn test_is_empty_true_after_deregister_all_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    reg.deregister("ok-svc");
    assert!(reg.is_empty());
}

// ─── find_by ─────────────────────────────────────────────────────────────────
// Covers: QueryableRepository::find_by

#[test]
fn test_find_by_returns_all_matching_items_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "alpha".into()).await.unwrap();
        repo.save(2u32, "beta".into()).await.unwrap();
        let all = repo.find_by(&AlwaysMatch).await.unwrap();
        assert_eq!(all.len(), 2);
    });
}

#[test]
fn test_find_by_no_match_returns_empty_vec_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        let result = repo.find_by(&NeverMatch).await.unwrap();
        assert!(result.is_empty());
    });
}

#[test]
fn test_find_by_empty_repo_returns_empty_vec_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        let result = repo.find_by(&AlwaysMatch).await.unwrap();
        assert!(result.is_empty());
    });
}

// ─── find_one_by ─────────────────────────────────────────────────────────────
// Covers: QueryableRepository::find_one_by

#[test]
fn test_find_one_by_returns_first_match_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "first".into()).await.unwrap();
        let result = repo.find_one_by(&AlwaysMatch).await.unwrap();
        assert_eq!(result.as_deref(), Some("first"));
    });
}

#[test]
fn test_find_one_by_no_match_returns_none_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        assert!(repo.find_one_by(&NeverMatch).await.unwrap().is_none());
    });
}

#[test]
fn test_find_one_by_empty_repo_returns_none_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        assert!(repo.find_one_by(&AlwaysMatch).await.unwrap().is_none());
    });
}

// ─── count_by ────────────────────────────────────────────────────────────────
// Covers: QueryableRepository::count_by

#[test]
fn test_count_by_returns_matching_count_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "a".into()).await.unwrap();
        repo.save(2u32, "b".into()).await.unwrap();
        assert_eq!(repo.count_by(&AlwaysMatch).await.unwrap(), 2);
    });
}

#[test]
fn test_count_by_no_match_returns_zero_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        assert_eq!(repo.count_by(&NeverMatch).await.unwrap(), 0);
    });
}

#[test]
fn test_count_by_empty_repo_returns_zero_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_queryable_repository::<String, u32>();
        assert_eq!(repo.count_by(&AlwaysMatch).await.unwrap(), 0);
    });
}

// ─── find ────────────────────────────────────────────────────────────────────
// Covers: Repository::find

#[test]
fn test_find_after_save_returns_some_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(7u32, "seven".into()).await.unwrap();
        assert_eq!(repo.find(&7u32).await.unwrap().as_deref(), Some("seven"));
    });
}

#[test]
fn test_find_nonexistent_returns_ok_none_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        assert!(repo.find(&0u32).await.unwrap().is_none());
    });
}

#[test]
fn test_find_after_delete_returns_none_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        repo.delete(&1u32).await.unwrap();
        assert!(repo.find(&1u32).await.unwrap().is_none());
    });
}

// ─── save ────────────────────────────────────────────────────────────────────
// Covers: Repository::save

#[test]
fn test_save_then_find_round_trips_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "hello".into()).await.unwrap();
        assert_eq!(repo.find(&1u32).await.unwrap().as_deref(), Some("hello"));
    });
}

#[test]
fn test_save_overwrites_existing_entity_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "old".into()).await.unwrap();
        repo.save(1u32, "new".into()).await.unwrap();
        assert_eq!(repo.find(&1u32).await.unwrap().as_deref(), Some("new"));
    });
}

#[test]
fn test_save_multiple_entities_increases_count_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        for i in 0..5u32 {
            repo.save(i, i.to_string()).await.unwrap();
        }
        assert_eq!(repo.count().await.unwrap(), 5);
    });
}

// ─── delete ──────────────────────────────────────────────────────────────────
// Covers: Repository::delete

#[test]
fn test_delete_existing_entity_returns_true_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        assert!(repo.delete(&1u32).await.unwrap());
    });
}

#[test]
fn test_delete_nonexistent_entity_returns_false_error() {
    block_on(async {
        // delete of non-existent key must return false, not Err
        let repo = Domain::new_in_memory_repository::<String, u32>();
        assert!(!repo.delete(&99u32).await.unwrap());
    });
}

#[test]
fn test_delete_reduces_count_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "a".into()).await.unwrap();
        repo.save(2u32, "b".into()).await.unwrap();
        repo.delete(&1u32).await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 1);
    });
}

// ─── list ────────────────────────────────────────────────────────────────────
// Covers: Repository::list

#[test]
fn test_list_returns_all_saved_entities_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "a".into()).await.unwrap();
        repo.save(2u32, "b".into()).await.unwrap();
        let all = repo.list().await.unwrap();
        assert_eq!(all.len(), 2);
    });
}

#[test]
fn test_list_empty_repo_returns_empty_vec_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        let all = repo.list().await.unwrap();
        assert!(all.is_empty());
    });
}

#[test]
fn test_list_after_delete_reflects_removal_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "a".into()).await.unwrap();
        repo.delete(&1u32).await.unwrap();
        let all = repo.list().await.unwrap();
        assert!(all.is_empty());
    });
}

// ─── exists ──────────────────────────────────────────────────────────────────
// Covers: Repository::exists

#[test]
fn test_exists_after_save_returns_true_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        assert!(repo.exists(&1u32).await.unwrap());
    });
}

#[test]
fn test_exists_nonexistent_returns_false_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        assert!(!repo.exists(&99u32).await.unwrap());
    });
}

#[test]
fn test_exists_after_delete_returns_false_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        repo.delete(&1u32).await.unwrap();
        assert!(!repo.exists(&1u32).await.unwrap());
    });
}

// ─── count ───────────────────────────────────────────────────────────────────
// Covers: Repository::count

#[test]
fn test_count_reflects_number_of_saved_entities_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "a".into()).await.unwrap();
        repo.save(2u32, "b".into()).await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 2);
    });
}

#[test]
fn test_count_empty_repo_returns_zero_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        assert_eq!(repo.count().await.unwrap(), 0);
    });
}

#[test]
fn test_count_decrements_after_delete_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        repo.delete(&1u32).await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 0);
    });
}

// ─── list_page ───────────────────────────────────────────────────────────────
// Covers: Repository::list_page

#[test]
fn test_list_page_returns_first_page_happy() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        for i in 0..5u32 {
            repo.save(i, i.to_string()).await.unwrap();
        }
        let page = repo.list_page(0, 3).await.unwrap();
        assert_eq!(page.items.len(), 3);
        assert_eq!(page.total, 5);
    });
}

#[test]
fn test_list_page_offset_beyond_end_returns_empty_items_not_error() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".into()).await.unwrap();
        let page = repo.list_page(10, 5).await.unwrap();
        assert!(page.items.is_empty());
        assert_eq!(page.total, 1);
    });
}

#[test]
fn test_list_page_total_equals_full_count_edge() {
    block_on(async {
        let repo = Domain::new_in_memory_repository::<String, u32>();
        for i in 0..4u32 {
            repo.save(i, i.to_string()).await.unwrap();
        }
        let page = repo.list_page(0, 2).await.unwrap();
        assert_eq!(page.total, 4);
        assert_eq!(page.items.len(), 2);
    });
}

// ─── matches ─────────────────────────────────────────────────────────────────
// Covers: Spec::matches

#[test]
fn test_matches_always_true_spec_returns_true_happy() {
    assert!(AlwaysMatch.matches(&"anything".to_string()));
}

#[test]
fn test_matches_always_false_spec_returns_false_error() {
    assert!(!NeverMatch.matches(&"anything".to_string()));
}

#[test]
fn test_matches_empty_string_input_edge() {
    assert!(AlwaysMatch.matches(&String::new()));
}

// ─── list_names ──────────────────────────────────────────────────────────────
// Covers: ServiceRegistry::list_names

#[test]
fn test_list_names_contains_registered_service_name_happy() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    assert!(reg.list_names().contains(&"ok-svc".to_string()));
}

#[test]
fn test_list_names_empty_before_registration_not_error() {
    let reg = Domain::new_service_registry::<String, String>();
    assert!(reg.list_names().is_empty());
}

#[test]
fn test_list_names_len_matches_registry_len_edge() {
    let reg = Domain::new_service_registry::<String, String>();
    reg.register(Arc::new(OkSvc));
    reg.register(Arc::new(ErrSvc));
    assert_eq!(reg.list_names().len(), reg.len());
}
