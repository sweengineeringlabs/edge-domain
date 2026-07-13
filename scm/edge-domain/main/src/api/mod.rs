//! API layer — domain aggregation re-exports.

#[cfg(test)]
mod tests;

// ── domain (never extracted; always internal) ─────────────────────────────────
mod domain;
#[cfg(feature = "command")]
pub use domain::{DirectCommandBusRequest, DirectCommandBusResponse};
pub use domain::{
    Domain, DomainError, DomainExtension, DomainExtensionHealthRequest, DomainRuntime,
    MemoryOutboundRegistry, NoopDomainExtension, OutboundDeregisterRequest,
    OutboundDeregisterResponse, OutboundGetRequest, OutboundGetResponse, OutboundIsEmptyRequest,
    OutboundIsEmptyResponse, OutboundLenRequest, OutboundLenResponse, OutboundNamesRequest,
    OutboundNamesResponse, OutboundRegisterRequest, OutboundRegisterResponse, OutboundRegistry,
};
#[cfg(feature = "event")]
pub use domain::{
    InProcessEventBusRequest, InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};

// ── spi ───────────────────────────────────────────────────────────────────────
mod spi;
pub use spi::{DomainAssemblyHook, NoopDomainAssemblyHook};

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBus;
#[cfg(feature = "command")]
pub use edge_domain_command::DirectCommandBus;

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_domain_event::Aggregate;
#[cfg(feature = "event")]
pub use edge_domain_event::AggregateApplyRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::AggregateApplyResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::AggregateIdentityRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::AggregateIdentityResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::DomainEvent;
#[cfg(feature = "event")]
pub use edge_domain_event::EventAggregateIdRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventAggregateIdResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusConfig;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusPublishRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusSubscribeRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusSubscribeResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventError;
#[cfg(feature = "event")]
pub use edge_domain_event::EventOccurredAtRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventOccurredAtResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventPublisher;
#[cfg(feature = "event")]
pub use edge_domain_event::EventPublisherPublishRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventSourceRecvNextRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventSourceRecvNextResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStore;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreAppendRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreAppendResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreError;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreLoadFromRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreLoadFromResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreLoadRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreLoadResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::EventTypeRequest;
#[cfg(feature = "event")]
pub use edge_domain_event::EventTypeResponse;
#[cfg(feature = "event")]
pub use edge_domain_event::MemoryEventStore;
#[cfg(feature = "event")]
pub use edge_domain_event::InProcessEventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::NoopEventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::NoopEventPublisher;

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_domain_handler::EchoHandler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::Handler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerRegistry;
#[cfg(feature = "handler")]
pub use edge_domain_handler::InProcessHandlerRegistry;

// ── projection ────────────────────────────────────────────────────────────────
#[cfg(feature = "projection")]
pub use edge_domain_projection::MemoryProjection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::Projection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionApplyRequest;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionReadModelRequest;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionReadModelResponse;

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_domain_query::DirectQueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryDispatchRequest;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryExecuteRequest;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryNameRequest;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryNameResponse;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryResultResponse;

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub use edge_domain_repository::AlwaysMatchSpec;
#[cfg(feature = "repository")]
pub use edge_domain_repository::CountByResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::MemoryRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::MatchingEntitiesResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::MatchingEntityResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::QueryableRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Repository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryCountResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryDeleteResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryExistsResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryFindResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryIdRequest;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryListPageRequest;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryListPageResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryListRequest;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryListResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositorySaveRequest;
#[cfg(feature = "repository")]
pub use edge_domain_repository::SpecMatchesRequest;
#[cfg(feature = "repository")]
pub use edge_domain_repository::SpecMatchesResponse;
#[cfg(feature = "repository")]
pub use edge_domain_repository::SpecRequest;

// ── saga ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_domain_saga::MemorySagaStore;
#[cfg(feature = "saga")]
pub use edge_domain_saga::Saga;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaStore;

// ── service ───────────────────────────────────────────────────────────────────
#[cfg(feature = "service")]
pub use edge_domain_service::Service;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistry;

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::MemorySnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::Snapshot;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStore;

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub use edge_domain_validator::Validator;
