//! Integration tests for the in-memory saga store implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Command, Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    Saga, SagaError, SagaGetRequest, SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest,
    SagaIsCompleteResponse, SagaRegisterRequest, SagaStore,
};
use edge_domain_command::ExecutionRequest;

#[derive(Clone)]
struct Pulse {
    id: String,
}
impl DomainEvent for Pulse {
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
struct Step;
impl Command for Step {
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct PulseSaga {
    steps: u32,
}
impl Saga for PulseSaga {
    type SagaId = String;
    type Event = Pulse;
    type Command = Step;
    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        self.steps += 1;
        Ok(SagaHandleResponse {
            commands: vec![Step],
        })
    }
    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.steps >= 2,
        })
    }
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_stores_and_retrieves_saga() {
    let mut store: Box<dyn SagaStore<SagaInstance = PulseSaga>> =
        Domain.new_in_memory_saga_store::<PulseSaga>();
    store
        .register(SagaRegisterRequest {
            id: "p1".to_string(),
            saga: PulseSaga::default(),
        })
        .unwrap();
    let id = "p1".to_string();
    let saga = store.get(SagaGetRequest { id: &id }).unwrap().saga;
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_rejects_duplicate_registration() {
    let mut store: Box<dyn SagaStore<SagaInstance = PulseSaga>> =
        Domain.new_in_memory_saga_store::<PulseSaga>();
    store
        .register(SagaRegisterRequest {
            id: "p1".to_string(),
            saga: PulseSaga::default(),
        })
        .unwrap();
    assert_eq!(
        store
            .register(SagaRegisterRequest {
                id: "p1".to_string(),
                saga: PulseSaga::default()
            })
            .unwrap_err(),
        SagaError::AlreadyRegistered("p1".to_string())
    );
}

/// @covers: new_in_memory_saga_store
#[test]
fn test_in_memory_saga_store_lookup_of_unknown_id_is_not_found() {
    let store: Box<dyn SagaStore<SagaInstance = PulseSaga>> =
        Domain.new_in_memory_saga_store::<PulseSaga>();
    let id = "unknown".to_string();
    assert_eq!(
        store.get(SagaGetRequest { id: &id }).unwrap_err(),
        SagaError::NotFound("unknown".to_string())
    );
}
