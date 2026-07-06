//! SAF layer — domain public facade.

#[cfg(test)]
mod tests;

mod aggregate_svc;
mod clock;
mod command;
mod domain;
mod entity_svc;
mod event;
mod handler;
mod policy;
mod policy_svc;
mod projection_svc;
mod query;
mod queryable_repository_svc;
mod repository;
mod repository_svc;
mod saga;
mod service;
mod snapshot;
mod spec_svc;
mod validator_svc;
mod value;
mod value_object_svc;

// ── sub-module re-exports (all api/ exports flow through _svc.rs inside each subdir) ─
pub use self::clock::*;
pub use self::command::*;
pub use self::domain::*;
pub use self::event::*;
pub use self::handler::*;
pub use self::policy::*;
pub use self::query::*;
pub use self::repository::*;
pub use self::saga::*;
pub use self::service::*;
pub use self::snapshot::*;
pub use self::value::*;

// ── top-level _svc.rs re-exports ─────────────────────────────────────────────
pub use self::aggregate_svc::*;
pub use self::entity_svc::*;
pub use self::policy_svc::*;
pub use self::projection_svc::*;
pub use self::queryable_repository_svc::*;
pub use self::repository_svc::*;
pub use self::spec_svc::*;
pub use self::validator_svc::*;
pub use self::value_object_svc::*;

// ── entity (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "entity")]
pub use edge_domain_entity::Entity;

// ── valueobject (sub-crate when feature enabled) ─────────────────────────────
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::NonEmptyString;
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::ValueObject;
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::ValueObjectError;

// ── clock (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "clock")]
pub use edge_domain_clock::Clock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::FixedClock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::SystemClock;

// ── validator (sub-crate when feature enabled; re-exported publicly via api::*) ─
#[cfg(feature = "validator")]
pub(crate) use edge_domain_validator::Validator;
#[cfg(feature = "validator")]
pub use edge_domain_validator::ValidatorError;

// ── policy (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "policy")]
pub use edge_domain_policy::CompositePolicy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::Policy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyError;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyEvaluateRequest;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyNameRequest;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyNameResponse;

// ── command (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "command")]
pub use edge_domain_command::Command;
pub use edge_domain_command::CommandBusBootstrap;
pub use edge_domain_command::CommandError;

// ── query (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_domain_query::Query;
pub use edge_domain_query::QueryError;

// ── snapshot (sub-crate when feature enabled) ─────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotAggregateIdRequest;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotAggregateIdResponse;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotError;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotLoadRequest;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotLoadResponse;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotSaveRequest;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotVersionRequest;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotVersionResponse;

// ── service (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceError;
#[cfg(feature = "service")]
pub use edge_domain_service::StdServiceRegistryFactory;

// ── repository (sub-crate when feature enabled) ───────────────────────────────
#[cfg(feature = "repository")]
pub use edge_domain_repository::Page;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryError;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Spec;
#[cfg(feature = "repository")]
pub use edge_domain_repository::QUERYABLE_REPOSITORY_SVC;
#[cfg(feature = "repository")]
pub use edge_domain_repository::QUERYABLE_REPOSITORY_SVC_FACTORY;
#[cfg(feature = "repository")]
pub use edge_domain_repository::REPOSITORY_SVC;
#[cfg(feature = "repository")]
pub use edge_domain_repository::REPOSITORY_SVC_FACTORY;
#[cfg(feature = "repository")]
pub use edge_domain_repository::SPEC_SVC;
#[cfg(feature = "repository")]
pub use edge_domain_repository::SPEC_SVC_FACTORY;

// ── handler (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerBootstrap;
pub use edge_domain_handler::HandlerContext;
pub use edge_domain_handler::HandlerError;
pub use edge_domain_handler::HandlerProvider;

// ── event (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_domain_event::ClosedEventSource;
pub use edge_domain_event::EventEnvelope;
pub use edge_domain_event::EventReceiver;
pub use edge_domain_event::EventSource;
pub use edge_domain_event::ExpectedVersion;

// ── pipeline (sub-crate when feature enabled) ────────────────────────────────
#[cfg(feature = "pipeline")]
pub use edge_pipeline::ContextMutationRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::Pipeline;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineBuilder;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineConfig;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineEmptinessRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineError;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineSvc;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::Step;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepCountRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepNameRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepNameResponse;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepRegistry as PipelineStepRegistry;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepRegistrySvc as PipelineStepRegistrySvc;
#[cfg(feature = "pipeline")]
pub(crate) use edge_pipeline::Validator as PipelineValidator;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::ValidatorSvc as PipelineValidatorSvc;

// ── projection (sub-crate when feature enabled) ───────────────────────────────
pub use edge_domain_projection::ProjectionError;
#[cfg(feature = "projection")]
pub use edge_domain_projection::TryDrainResponse;

// ── saga (sub-crate when feature enabled) ─────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaError;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaGetRequest;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaGetResponse;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaHandleRequest;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaHandleResponse;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaIsCompleteRequest;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaIsCompleteResponse;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaRegisterRequest;

// ── registry (opt-in; NOT in default features) ────────────────────────────────
#[cfg(feature = "registry")]
pub use edge_domain_registry::InMemoryRegistry;
#[cfg(feature = "registry")]
pub use edge_domain_registry::Registry;
#[cfg(feature = "registry")]
pub use edge_domain_registry::RegistryBootstrap;
#[cfg(feature = "registry")]
pub use edge_domain_registry::RegistryError;

// ── lifecycle (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::Lifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleError;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleIsInRequest;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleIsInResponse;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleStateRequest;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleStateResponse;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleTransitionRequest;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::ManagedLifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::PermissivePolicy;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::TransitionAllowedRequest;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::TransitionAllowedResponse;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::TransitionPolicy;

// ── security (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "security")]
pub use edge_domain_security::Principal;
pub use edge_domain_security::Security;
pub use edge_domain_security::SecurityBootstrap;
pub use edge_domain_security::SecurityError;

// ── observer (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "observer")]
pub use edge_domain_observer::Observer;
#[cfg(feature = "observer")]
pub use edge_domain_observer::ObserverBootstrap;
