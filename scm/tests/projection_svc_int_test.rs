//! SAF facade integration tests — `Projection` trait is exported from the
//! crate root and implementable by downstream consumers.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_domain::{DomainEvent, EventEnvelope, Projection};

#[derive(Clone)]
struct AccountCredited {
    id: String,
    cents: u64,
}

impl DomainEvent for AccountCredited {
    fn event_type(&self) -> &str {
        "account.credited"
    }
    fn aggregate_id(&self) -> &str {
        &self.id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
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

    fn apply(&mut self, event: &EventEnvelope<Self::Event>) {
        self.model.balance_cents += event.event.cents;
        self.model.events_seen += 1;
    }

    fn read_model(&self) -> &Self::ReadModel {
        &self.model
    }
}

fn envelope(seq: u64, cents: u64) -> EventEnvelope<AccountCredited> {
    EventEnvelope {
        aggregate_id: "acct-1".to_string(),
        sequence: seq,
        occurred_at: SystemTime::now(),
        event: AccountCredited {
            id: "acct-1".to_string(),
            cents,
        },
    }
}

/// @covers: Projection::apply
/// @covers: Projection::read_model
#[test]
fn test_projection_svc_facade_read_model_reflects_applied_events() {
    let mut p = BalanceProjection::default();
    p.apply(&envelope(1, 500));
    p.apply(&envelope(2, 250));
    assert_eq!(p.read_model().balance_cents, 750);
    assert_eq!(p.read_model().events_seen, 2);
}

/// @covers: Projection::read_model
#[test]
fn test_projection_svc_facade_read_model_is_initial_before_any_event() {
    let p = BalanceProjection::default();
    assert_eq!(p.read_model().balance_cents, 0);
    assert_eq!(p.read_model().events_seen, 0);
}
