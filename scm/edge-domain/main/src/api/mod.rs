//! API layer — domain aggregation re-exports.

#![allow(unused_imports)]

// ── domain (never extracted; always internal) ─────────────────────────────────
mod domain;
pub use domain::{Domain, DomainBootstrap, DomainError, DomainExtension, NoopDomainExtension, OutboundRegistry};

// ── spi ───────────────────────────────────────────────────────────────────────
mod spi;
pub use spi::DomainSpi;

// ── clock ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "clock")]
pub mod clock {
    pub use edge_domain_clock::Clock;
    pub use edge_domain_clock::ClockBootstrap;
    pub use edge_domain_clock::ClockError;
    pub use edge_domain_clock::FixedClock;
    pub use edge_domain_clock::SystemClock;
    pub mod traits {
        pub use super::Clock;
    }
    pub mod types {
        pub use super::FixedClock;
        pub use super::SystemClock;
    }
}
#[cfg(feature = "clock")]
pub use clock::{Clock, ClockBootstrap, FixedClock, SystemClock};

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub mod validator {
    pub use edge_domain_validator::AlwaysValid;
    pub use edge_domain_validator::Validator;
    pub use edge_domain_validator::ValidatorBootstrap;
    pub use edge_domain_validator::ValidatorError;
    pub mod traits {
        pub use super::Validator;
    }
}
#[cfg(feature = "validator")]
pub use validator::Validator;

// ── policy ────────────────────────────────────────────────────────────────────
#[cfg(feature = "policy")]
pub mod policy {
    pub use edge_domain_policy::CompositePolicy;
    pub use edge_domain_policy::Policy;
    pub use edge_domain_policy::PolicyBootstrap;
    pub use edge_domain_policy::PolicyViolation;
    pub mod traits {
        pub use super::Policy;
    }
    pub mod types {
        pub use super::CompositePolicy;
        pub use super::PolicyViolation;
    }
}
#[cfg(feature = "policy")]
pub use policy::{CompositePolicy, Policy, PolicyBootstrap, PolicyViolation};

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub mod command {
    pub use edge_domain_command::Command;
    pub use edge_domain_command::CommandBus;
    pub use edge_domain_command::CommandBusBootstrap;
    pub use edge_domain_command::CommandError;
    pub use edge_domain_command::DirectCommandBus;
}
#[cfg(feature = "command")]
pub use command::{Command, CommandBus, CommandBusBootstrap, CommandError, DirectCommandBus};

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub mod query {
    pub use edge_domain_query::DirectQueryBus;
    pub use edge_domain_query::Query;
    pub use edge_domain_query::QueryBus;
    pub use edge_domain_query::QueryBusBootstrap;
    pub use edge_domain_query::QueryError;
}
#[cfg(feature = "query")]
pub use query::{DirectQueryBus, Query, QueryBus, QueryBusBootstrap, QueryError};

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub mod snapshot {
    pub use edge_domain_snapshot::InMemorySnapshotStore;
    pub use edge_domain_snapshot::Snapshot;
    pub use edge_domain_snapshot::SnapshotError;
    pub use edge_domain_snapshot::SnapshotStore;
    pub use edge_domain_snapshot::SnapshotStoreBootstrap;
    pub mod traits {
        pub use super::Snapshot;
        pub use super::SnapshotStore;
    }
    pub mod errors {
        pub use super::SnapshotError;
    }
}
#[cfg(feature = "snapshot")]
pub use snapshot::{Snapshot, SnapshotError, SnapshotStore};

// ── service ───────────────────────────────────────────────────────────────────
#[cfg(feature = "service")]
pub mod service {
    pub use edge_domain_service::Service;
    pub use edge_domain_service::ServiceError;
    pub use edge_domain_service::ServiceRegistryBootstrap;
    pub use edge_domain_service::ServiceRegistryImpl as ServiceRegistry;
    pub use edge_domain_service::ServiceRegistry as ServiceRegistryImpl;
    pub mod types {
        pub use edge_domain_service::ServiceRegistry;
    }
}
#[cfg(feature = "service")]
pub use service::{Service, ServiceError, ServiceRegistry, ServiceRegistryImpl};

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub mod repository {
    pub use edge_domain_repository::InMemoryRepository;
    pub use edge_domain_repository::Page;
    pub use edge_domain_repository::QueryableRepository;
    pub use edge_domain_repository::Repository;
    pub use edge_domain_repository::RepositoryBootstrap;
    pub use edge_domain_repository::RepositoryError;
    pub use edge_domain_repository::Spec;
}
#[cfg(feature = "repository")]
pub use repository::{
    InMemoryRepository, Page, QueryableRepository, Repository, RepositoryBootstrap, RepositoryError,
    Spec,
};

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub mod handler {
    pub use edge_domain_handler::EchoHandler;
    pub use edge_domain_handler::Handler;
    pub use edge_domain_handler::HandlerBootstrap;
    pub use edge_domain_handler::HandlerContext;
    pub use edge_domain_handler::HandlerError;
    pub use edge_domain_handler::HandlerProvider;
    pub use edge_domain_handler::HandlerRegistry;
    pub use edge_domain_handler::InProcessHandlerRegistry;
}
#[cfg(feature = "handler")]
pub use handler::{
    EchoHandler, Handler, HandlerBootstrap, HandlerContext, HandlerError, HandlerProvider,
    HandlerRegistry, InProcessHandlerRegistry,
};

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub mod event {
    pub use edge_domain_event::Aggregate;
    pub use edge_domain_event::ClosedEventSource;
    pub use edge_domain_event::DomainEvent;
    pub use edge_domain_event::EventBootstrap;
    pub use edge_domain_event::EventBus;
    pub use edge_domain_event::EventBusConfig;
    pub use edge_domain_event::EventEnvelope;
    pub use edge_domain_event::EventError;
    pub use edge_domain_event::EventPublisher;
    pub use edge_domain_event::EventReceiver;
    pub use edge_domain_event::EventSource;
    pub use edge_domain_event::EventStore;
    pub use edge_domain_event::EventStoreError;
    pub use edge_domain_event::ExpectedVersion;
    pub use edge_domain_event::InMemoryEventStore;
    pub use edge_domain_event::InProcessEventBus;
    pub use edge_domain_event::NoopEventBus;
    pub use edge_domain_event::NoopEventPublisher;
    pub mod errors {
        pub use super::EventError;
        pub use super::EventStoreError;
    }
    pub mod traits {
        pub mod domain_event {
            pub use super::super::DomainEvent;
        }
    }
}
#[cfg(feature = "event")]
pub use event::{
    Aggregate, ClosedEventSource, DomainEvent, EventBootstrap, EventBus, EventBusConfig,
    EventEnvelope, EventError, EventPublisher, EventReceiver, EventSource, EventStore,
    EventStoreError, ExpectedVersion, InMemoryEventStore, InProcessEventBus, NoopEventBus,
    NoopEventPublisher,
};

// ── projection ────────────────────────────────────────────────────────────────
#[cfg(feature = "projection")]
pub mod projection {
    pub use edge_domain_projection::InMemoryProjection;
    pub use edge_domain_projection::Projection;
    pub use edge_domain_projection::ProjectionBootstrap;
    pub use edge_domain_projection::ProjectionError;
}
#[cfg(feature = "projection")]
pub use projection::Projection;

// ── saga ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "saga")]
pub mod saga {
    pub use edge_domain_saga::InMemorySagaStore;
    pub use edge_domain_saga::Saga;
    pub use edge_domain_saga::SagaBootstrap;
    pub use edge_domain_saga::SagaError;
    pub use edge_domain_saga::SagaStore;
    pub mod traits {
        pub use super::Saga;
        pub use super::SagaStore;
    }
    pub mod errors {
        pub use super::SagaError;
    }
}
#[cfg(feature = "saga")]
pub use saga::{InMemorySagaStore, Saga, SagaError, SagaStore};
