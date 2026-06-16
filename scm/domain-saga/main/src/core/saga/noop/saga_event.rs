use edge_domain_event::DomainEvent;

use crate::api::saga::types::NoopSagaEvent;

impl DomainEvent for NoopSagaEvent {}
