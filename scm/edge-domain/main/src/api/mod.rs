//! API layer — domain execution-unit contracts.
//!
//! Multi-theme layout (ADR-007): each theme owns its `traits/ types/ error/
//! vo/` subdirs. Cross-theme items live at the `api/` level in `traits/` and
//! `types/`. The `api/` surface is technology-neutral (ADR-008) — concrete
//! external-library implementations live under `spi/`.

#![allow(unused_imports)]

// ── domain (never extracted; always internal) ─────────────────────────────────
pub mod domain;

// ── entity ────────────────────────────────────────────────────────────────────
#[cfg(not(feature = "entity"))]
pub mod entity;

// ── valueobject ───────────────────────────────────────────────────────────────
#[cfg(not(feature = "valueobject"))]
pub mod valueobject;

// ── clock ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "clock")]
pub mod clock {
    pub use edge_domain_clock::Clock;
    pub use edge_domain_clock::ClockError;
    pub use edge_domain_clock::ClockFactory;
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
#[cfg(not(feature = "clock"))]
pub mod clock;

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub mod validator {
    pub use edge_domain_validator::AlwaysValid;
    pub use edge_domain_validator::Validator;
    pub use edge_domain_validator::ValidatorError;
    pub use edge_domain_validator::ValidatorFactory;
    pub mod traits {
        pub use super::Validator;
    }
}
#[cfg(not(feature = "validator"))]
pub mod validator;

// ── policy ────────────────────────────────────────────────────────────────────
#[cfg(feature = "policy")]
pub mod policy {
    pub use edge_domain_policy::CompositePolicy;
    pub use edge_domain_policy::Policy;
    pub use edge_domain_policy::PolicyFactory;
    pub use edge_domain_policy::PolicyViolation;
    pub mod traits {
        pub use super::Policy;
    }
    pub mod types {
        pub use super::CompositePolicy;
        pub use super::PolicyViolation;
    }
}
#[cfg(not(feature = "policy"))]
pub mod policy;

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub mod command {
    pub use edge_domain_command::Command;
    pub use edge_domain_command::CommandBus;
    pub use edge_domain_command::CommandBusFactory;
    pub use edge_domain_command::CommandError;
    pub use edge_domain_command::DirectCommandBus;
}
#[cfg(not(feature = "command"))]
pub mod command;

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub mod query {
    pub use edge_domain_query::DirectQueryBus;
    pub use edge_domain_query::Query;
    pub use edge_domain_query::QueryBus;
    pub use edge_domain_query::QueryBusFactory;
    pub use edge_domain_query::QueryError;
}
#[cfg(not(feature = "query"))]
pub mod query;

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub mod snapshot {
    pub use edge_domain_snapshot::InMemorySnapshotStore;
    pub use edge_domain_snapshot::Snapshot;
    pub use edge_domain_snapshot::SnapshotError;
    pub use edge_domain_snapshot::SnapshotStore;
    pub use edge_domain_snapshot::SnapshotStoreFactory;
    pub mod traits {
        pub use super::Snapshot;
        pub use super::SnapshotStore;
    }
    pub mod errors {
        pub use super::SnapshotError;
    }
}
#[cfg(not(feature = "snapshot"))]
pub mod snapshot;

// ── service ───────────────────────────────────────────────────────────────────
#[cfg(feature = "service")]
pub mod service {
    pub use edge_domain_service::Service;
    pub use edge_domain_service::ServiceError;
    pub use edge_domain_service::ServiceRegistryFactory;
    // In the sub-crate: ServiceRegistryImpl = the trait, ServiceRegistry = the struct.
    // Mirror the umbrella's internal naming: ServiceRegistry (top-level) = trait.
    pub use edge_domain_service::ServiceRegistryImpl as ServiceRegistry;
    pub mod types {
        pub use edge_domain_service::ServiceRegistry; // concrete struct
    }
}
#[cfg(not(feature = "service"))]
pub mod service;

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub mod repository {
    pub use edge_domain_repository::InMemoryRepository;
    pub use edge_domain_repository::Page;
    pub use edge_domain_repository::QueryableRepository;
    pub use edge_domain_repository::Repository;
    pub use edge_domain_repository::RepositoryError;
    pub use edge_domain_repository::RepositoryFactory;
    pub use edge_domain_repository::Spec;
}
#[cfg(not(feature = "repository"))]
pub mod repository;

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub mod handler {
    pub use edge_domain_handler::EchoHandler;
    pub use edge_domain_handler::Handler;
    pub use edge_domain_handler::HandlerError;
    pub use edge_domain_handler::HandlerFactory;
    pub use edge_domain_handler::HandlerProvider;
    pub use edge_domain_handler::HandlerRegistry;
    pub use edge_domain_handler::InProcessHandlerRegistry;
}
#[cfg(not(feature = "handler"))]
pub mod handler;

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub mod event {
    pub use edge_domain_event::Aggregate;
    pub use edge_domain_event::ClosedEventSource;
    pub use edge_domain_event::DomainEvent;
    pub use edge_domain_event::EventBus;
    pub use edge_domain_event::EventBusConfig;
    pub use edge_domain_event::EventEnvelope;
    pub use edge_domain_event::EventError;
    pub use edge_domain_event::EventFactory;
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
    /// Sub-paths used by `core/` and `spi/`.
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
#[cfg(not(feature = "event"))]
pub mod event;

// ── projection ────────────────────────────────────────────────────────────────
#[cfg(feature = "projection")]
pub mod projection {
    pub use edge_domain_projection::InMemoryProjection;
    pub use edge_domain_projection::Projection;
    pub use edge_domain_projection::ProjectionError;
    pub use edge_domain_projection::ProjectionFactory;
}
#[cfg(not(feature = "projection"))]
pub mod projection;

// ── saga ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "saga")]
pub mod saga {
    pub use edge_domain_saga::InMemorySagaStore;
    pub use edge_domain_saga::Saga;
    pub use edge_domain_saga::SagaError;
    pub use edge_domain_saga::SagaFactory;
    pub use edge_domain_saga::SagaStore;
    pub mod traits {
        pub use super::Saga;
        pub use super::SagaStore;
    }
    pub mod errors {
        pub use super::SagaError;
    }
}
#[cfg(not(feature = "saga"))]
pub mod saga;
