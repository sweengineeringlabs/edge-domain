//! SAF facade integration tests — `Projection` trait is exported from the
//! crate root and implementable by downstream consumers.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_application::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventOccurredAtRequest, EventOccurredAtResponse, EventTypeRequest, EventTypeResponse,
    Projection, ProjectionApplyRequest, ProjectionError, ProjectionReadModelRequest,
    ProjectionReadModelResponse,
};

#[derive(Clone)]
struct AccountCredited {
    id: String,
    cents: u64,
}

impl DomainEvent for AccountCredited {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "account.credited",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::now(),
        })
    }
}

/// A user-defined read model and projection — proves the trait is implementable
/// outside the crate, not only via the in-memory reference impl.
#[derive(Default)]
struct BalanceReadModel {
    balance_cents: u64,
    events_seen: u64,
}

#[derive(Default)]
struct BalanceProjection {
    model: BalanceReadModel,
}

impl Projection for BalanceProjection {
    type Event = AccountCredited;
    type ReadModel = BalanceReadModel;

    fn apply(
        &mut self,
        req: ProjectionApplyRequest<'_, Self::Event>,
    ) -> Result<(), ProjectionError> {
        self.model.balance_cents += req.event.cents;
        self.model.events_seen += 1;
        Ok(())
    }

    fn read_model(
        &self,
        _req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, Self::ReadModel>, ProjectionError> {
        Ok(ProjectionReadModelResponse {
            read_model: &self.model,
        })
    }
}

fn event(cents: u64) -> AccountCredited {
    AccountCredited {
        id: "acct-1".to_string(),
        cents,
    }
}

/// @covers: Projection::apply
/// @covers: Projection::read_model
#[test]
fn test_projection_svc_facade_read_model_reflects_applied_events() {
    let mut p = BalanceProjection::default();
    p.apply(ProjectionApplyRequest { event: &event(500) })
        .expect("apply should succeed");
    p.apply(ProjectionApplyRequest { event: &event(250) })
        .expect("apply should succeed");
    let model = p
        .read_model(ProjectionReadModelRequest)
        .expect("read_model should succeed")
        .read_model;
    assert_eq!(model.balance_cents, 750);
    assert_eq!(model.events_seen, 2);
}

/// @covers: Projection::read_model
#[test]
fn test_projection_svc_facade_read_model_is_initial_before_any_event() {
    let p = BalanceProjection::default();
    let model = p
        .read_model(ProjectionReadModelRequest)
        .expect("read_model should succeed")
        .read_model;
    assert_eq!(model.balance_cents, 0);
    assert_eq!(model.events_seen, 0);
}
