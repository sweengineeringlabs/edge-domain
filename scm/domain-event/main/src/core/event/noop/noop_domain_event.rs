//! `impl DomainEvent for NoopDomainEvent`.

use crate::api::event::traits::DomainEvent;
use crate::api::event::types::NoopDomainEvent;

impl DomainEvent for NoopDomainEvent {}
