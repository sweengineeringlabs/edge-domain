//! Example: Implementing a custom EventBus for specialized requirements.
//!
//! Demonstrates:
//! - EventBus trait is public in edge/domain API → consumers can implement it
//! - Custom implementations for domain-specific needs (filtering, persistence, etc.)
//! - Consumer returns Arc<dyn EventBus> to maintain factory pattern consistency
//!
//! Run:
//!     cargo run -p edge-domain --example custom_event_bus
//!
//! SEA constraint: all imports come from the edge_domain SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::DomainRuntime;
use edge_domain::InProcessEventBusRequest;
use edge_domain::{
    Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventBus,
    EventBusConfig, EventBusPublishRequest, EventBusSubscribeRequest, EventBusSubscribeResponse,
    EventError, EventOccurredAtRequest, EventOccurredAtResponse, EventTypeRequest,
    EventTypeResponse,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
struct AuditedEvent {
    event_type: String,
    timestamp: u64,
}

impl DomainEvent for AuditedEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: &self.event_type,
        })
    }

    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "audit-log",
        })
    }

    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: std::time::SystemTime::UNIX_EPOCH
                + std::time::Duration::from_secs(self.timestamp),
        })
    }
}

/// Custom EventBus that adds audit logging on top of an underlying bus.
/// This is what a consumer domain might implement for compliance/traceability.
pub struct AuditingEventBus {
    inner: Arc<dyn EventBus>,
    audit_log: Arc<Mutex<Vec<String>>>,
}

impl AuditingEventBus {
    pub fn new(inner: Arc<dyn EventBus>) -> Self {
        Self {
            inner,
            audit_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Inspect the audit log (consumer-specific capability)
    pub fn audit_log(&self) -> Vec<String> {
        self.audit_log.lock().unwrap().clone()
    }
}

impl EventBus for AuditingEventBus {
    fn publish(&self, req: EventBusPublishRequest) -> BoxFuture<'_, Result<(), EventError>> {
        let event = req.event;
        let audit_log = Arc::clone(&self.audit_log);
        let event_type = event
            .event_type(EventTypeRequest)
            .map(|r| r.event_type.to_string())
            .unwrap_or_default();
        let aggregate_id = event
            .aggregate_id(EventAggregateIdRequest)
            .map(|r| r.aggregate_id.to_string())
            .unwrap_or_default();
        let inner = Arc::clone(&self.inner);

        Box::pin(async move {
            // Custom behavior: log to audit trail
            let entry = format!(
                "[{}] {} → {}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                event_type,
                aggregate_id
            );

            audit_log.lock().unwrap().push(entry);

            // Delegate to underlying bus
            inner.publish(EventBusPublishRequest { event }).await
        })
    }

    fn subscribe(
        &self,
        req: EventBusSubscribeRequest,
    ) -> Result<EventBusSubscribeResponse, EventError> {
        // Delegate subscription to underlying bus
        self.inner.subscribe(req)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Custom EventBus Implementation Pattern ===\n");

    // Create the underlying event bus (edge/domain factory)
    println!("1. Creating underlying tokio event bus...");
    let config = EventBusConfig::default();
    let underlying_bus = Domain
        .in_process_event_bus(InProcessEventBusRequest { config })?
        .bus;
    println!("   ✓ Underlying bus created\n");

    // Consumer implements their own EventBus by wrapping/decorating
    println!("2. Wrapping with custom AuditingEventBus...");
    let bus = Arc::new(AuditingEventBus::new(underlying_bus));
    println!("   ✓ Custom EventBus ready\n");

    // Publish events
    println!("3. Publishing events through custom bus...");
    for i in 1..=3 {
        let event = Arc::new(AuditedEvent {
            event_type: format!("order.state_changed.phase_{i}"),
            timestamp: (1000 + i) as u64,
        });
        bus.publish(EventBusPublishRequest { event }).await?;
        println!("   ✓ Event {i} published");
    }
    println!();

    println!("=== Key Pattern Points ===");
    println!("✓ EventBus is public in edge/domain API");
    println!("✓ Consumer can implement/decorate for custom needs (audit, filtering, etc.)");
    println!("✓ Consumer returns Arc<dyn EventBus> (consistent with edge/domain)");
    println!("✓ No trait leakage — consumers don't force imports on their own users");
    println!("✓ Extensible: framework + consumers all follow same pattern");

    Ok(())
}
