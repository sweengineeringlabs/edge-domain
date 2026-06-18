use edge_domain_event::DomainEvent;

use crate::api::NoopSagaEvent;

impl DomainEvent for NoopSagaEvent {}
