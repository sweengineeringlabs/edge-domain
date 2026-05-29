//! Example: Seamless EventBus consumer — Arc<dyn EventBus> pattern.
//!
//! Demonstrates:
//! - Using edge/domain EventBus factories WITHOUT importing EventBus trait
//! - Factories return ready-to-use Arc<dyn EventBus> (trait object)
//! - Passes rule 207 (saf_no_leaked_trait_return)
//!
//! Run:
//!     cargo run -p edge-domain --example event_bus_consumer
//!
//! SEA constraint: all imports come from the edge_domain SAF surface.

use edge_domain::tokio_event_bus;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
struct OrderEvent {
    order_id: String,
    status: String,
}

impl edge_domain::DomainEvent for OrderEvent {
    fn event_type(&self) -> &str {
        "order.status_changed"
    }

    fn aggregate_id(&self) -> &str {
        &self.order_id
    }

    fn occurred_at(&self) -> std::time::SystemTime {
        std::time::SystemTime::now()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Arc<dyn EventBus> Seamless Consumer Pattern ===\n");

    // KEY: No EventBus import needed — just call the factory
    println!("1. Creating event bus via factory...");
    let config = edge_domain::EventBusConfig::default();
    let event_bus = tokio_event_bus(config);
    println!("   ✓ Got Arc<dyn EventBus> without importing trait\n");

    // Use the bus — no type knowledge of concrete implementation
    println!("2. Publishing events...");
    for i in 1..=3 {
        let event = Arc::new(OrderEvent {
            order_id: format!("order-{i}"),
            status: "processing".to_string(),
        });
        event_bus.publish(event).await?;
        println!("   ✓ Event {i} published");
    }
    println!();

    // Real-world service using the factory result
    println!("3. Service using factory result...");
    let service = OrderService { bus: event_bus };
    service.update_order("order-123", "completed").await?;
    println!();

    println!("=== Why This Works ===");
    println!("✓ Seamless: Arc<dyn EventBus> is self-contained");
    println!("✓ No imports: Consumer doesn't need EventBus trait");
    println!("✓ Extensible: EventBus is still public in API (if consumer wants to impl)");
    println!("✓ Compliant: Passes rule 207 (no trait leakage)");
    println!("✓ Safe: Arc enables thread-safe sharing");

    Ok(())
}

/// Example service receiving Arc<dyn EventBus> from factory.
struct OrderService {
    bus: Arc<dyn edge_domain::EventBus>,
}

impl OrderService {
    async fn update_order(
        &self,
        order_id: &str,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = Arc::new(OrderEvent {
            order_id: order_id.to_string(),
            status: status.to_string(),
        });
        self.bus.publish(event).await?;
        println!("   Service published: {order_id} → {status}");
        Ok(())
    }
}
