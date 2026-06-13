//! `impl Aggregate for NoopAggregate`.

use crate::api::event::traits::Aggregate;
use crate::api::event::types::{NoopAggregate, NoopDomainEvent};

impl Aggregate for NoopAggregate {
    type Event = NoopDomainEvent;
}
