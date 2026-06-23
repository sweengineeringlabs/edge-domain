//! API layer — domain aggregation re-exports.

// ── domain (never extracted; always internal) ─────────────────────────────────
mod domain;
pub use domain::{
    Domain, DomainBootstrap, DomainError, DomainExtension, NoopDomainExtension, OutboundRegistry,
};

// ── spi ───────────────────────────────────────────────────────────────────────
mod spi;
pub use spi::DomainSpi;

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBus;

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_domain_event::Aggregate;
#[cfg(feature = "event")]
pub use edge_domain_event::DomainEvent;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::EventPublisher;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStore;

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_domain_handler::Handler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerRegistry;

// ── projection ────────────────────────────────────────────────────────────────
#[cfg(feature = "projection")]
pub use edge_domain_projection::Projection;

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBus;

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub use edge_domain_repository::QueryableRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Repository;

// ── saga ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_domain_saga::Saga;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaStore;

// ── service ───────────────────────────────────────────────────────────────────
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistry;

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::Snapshot;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStore;

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub use edge_domain_validator::Validator;
