//! Integration tests for `SagaStore` and the `new_in_memory_saga_store`
//! SAF factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Command, Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    Saga, SagaError, SagaGetRequest, SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest,
    SagaIsCompleteResponse, SagaRegisterRequest, SagaStore,
};
use edge_domain_command::ExecutionRequest;

#[derive(Clone)]
struct Tick {
    id: String,
}
impl DomainEvent for Tick {
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
}

#[derive(Clone)]
struct Advance;
impl Command for Advance {
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct CounterSaga {
    ticks: u32,
}
impl Saga for CounterSaga {
    type SagaId = String;
    type Event = Tick;
    type Command = Advance;
    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        self.ticks += 1;
        Ok(SagaHandleResponse {
            commands: vec![Advance],
        })
    }
    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.ticks >= 3,
        })
    }
}

fn store() -> Box<dyn SagaStore<SagaInstance = CounterSaga>> {
    Domain::new_in_memory_saga_store::<CounterSaga>()
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_register_and_get_round_trip_happy() {
    let mut s = store();
    s.register(SagaRegisterRequest {
        id: "s1".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    let id = "s1".to_string();
    assert!(s.get(SagaGetRequest { id: &id }).is_ok());
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_lookup_on_empty_returns_not_found_error() {
    let s = store();
    let id = "absent".to_string();
    assert_eq!(
        s.get(SagaGetRequest { id: &id }).unwrap_err(),
        SagaError::NotFound("absent".to_string())
    );
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_new_in_memory_saga_store_starts_empty_edge() {
    let s = store();
    let id = "anything".to_string();
    assert!(s.get(SagaGetRequest { id: &id }).is_err());
}

/// @covers: SagaStore::register
#[test]
fn test_register_new_id_returns_ok_happy() {
    let mut s = store();
    assert_eq!(
        s.register(SagaRegisterRequest {
            id: "order-1".to_string(),
            saga: CounterSaga::default()
        }),
        Ok(())
    );
}

/// @covers: SagaStore::register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut s = store();
    s.register(SagaRegisterRequest {
        id: "order-1".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    assert_eq!(
        s.register(SagaRegisterRequest {
            id: "order-1".to_string(),
            saga: CounterSaga::default()
        })
        .unwrap_err(),
        SagaError::AlreadyRegistered("order-1".to_string())
    );
}

/// @covers: SagaStore::register
#[test]
fn test_register_distinct_ids_are_independent_edge() {
    let mut s = store();
    s.register(SagaRegisterRequest {
        id: "a".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    s.register(SagaRegisterRequest {
        id: "b".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    let a = "a".to_string();
    let b = "b".to_string();
    assert!(s.get(SagaGetRequest { id: &a }).is_ok());
    assert!(s.get(SagaGetRequest { id: &b }).is_ok());
}

/// @covers: SagaStore::get
#[test]
fn test_get_registered_id_returns_saga_happy() {
    let mut s = store();
    s.register(SagaRegisterRequest {
        id: "s1".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    let id = "s1".to_string();
    let saga = s.get(SagaGetRequest { id: &id }).unwrap().saga;
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: SagaStore::get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let s = store();
    let id = "ghost".to_string();
    assert_eq!(
        s.get(SagaGetRequest { id: &id }).unwrap_err(),
        SagaError::NotFound("ghost".to_string())
    );
}

/// @covers: SagaStore::get
#[test]
fn test_get_after_registering_other_id_still_not_found_edge() {
    let mut s = store();
    s.register(SagaRegisterRequest {
        id: "present".to_string(),
        saga: CounterSaga::default(),
    })
    .unwrap();
    let id = "missing".to_string();
    assert!(s.get(SagaGetRequest { id: &id }).is_err());
}
